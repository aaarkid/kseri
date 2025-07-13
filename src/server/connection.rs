use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::server::messages::{ClientMessage, ServerMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

pub struct Connection {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub player_name: Option<String>,
    pub last_ping: Instant,
    tx: mpsc::UnboundedSender<ServerMessage>,
}

impl Connection {
    pub fn new(id: Uuid, tx: mpsc::UnboundedSender<ServerMessage>) -> Self {
        Self {
            id,
            session_id: None,
            player_name: None,
            last_ping: Instant::now(),
            tx,
        }
    }

    pub async fn send(&self, message: ServerMessage) -> Result<(), &'static str> {
        self.tx.send(message).map_err(|_| "Failed to send message")
    }

    pub fn update_ping(&mut self) {
        self.last_ping = Instant::now();
    }

    pub fn is_timed_out(&self) -> bool {
        self.last_ping.elapsed() > CLIENT_TIMEOUT
    }
}

pub struct ConnectionManager {
    connections: Arc<DashMap<Uuid, Connection>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }

    pub fn add_connection(&self, id: Uuid, connection: Connection) {
        self.connections.insert(id, connection);
    }

    pub fn remove_connection(&self, id: &Uuid) -> Option<Connection> {
        self.connections.remove(id).map(|(_, conn)| conn)
    }

    pub fn get_connection(&self, id: &Uuid) -> Option<dashmap::mapref::one::Ref<Uuid, Connection>> {
        self.connections.get(id)
    }

    pub fn get_connection_mut(&self, id: &Uuid) -> Option<dashmap::mapref::one::RefMut<Uuid, Connection>> {
        self.connections.get_mut(id)
    }

    pub fn cleanup_timed_out(&self) -> Vec<Uuid> {
        let mut timed_out = Vec::new();
        
        self.connections.retain(|id, conn| {
            if conn.is_timed_out() {
                timed_out.push(*id);
                false
            } else {
                true
            }
        });
        
        timed_out
    }

    pub fn get_connections_count(&self) -> usize {
        self.connections.len()
    }
}

pub struct ConnectionHandler {
    id: Uuid,
    ws_stream: WebSocketStream<TcpStream>,
    rx: mpsc::UnboundedReceiver<ServerMessage>,
    manager: Arc<ConnectionManager>,
    sessions: Arc<crate::server::session::SessionManager>,
}

impl ConnectionHandler {
    pub fn new(
        id: Uuid,
        ws_stream: WebSocketStream<TcpStream>,
        rx: mpsc::UnboundedReceiver<ServerMessage>,
        manager: Arc<ConnectionManager>,
        sessions: Arc<crate::server::session::SessionManager>,
    ) -> Self {
        Self {
            id,
            ws_stream,
            rx,
            manager,
            sessions,
        }
    }

    pub async fn run(mut self) {
        info!("Connection {} established", self.id);
        
        let mut heartbeat = time::interval(HEARTBEAT_INTERVAL);
        
        loop {
            tokio::select! {
                msg = self.ws_stream.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Err(e) = self.handle_text_message(text).await {
                                error!("Error handling message: {}", e);
                            }
                        }
                        Some(Ok(Message::Binary(_))) => {
                            warn!("Binary messages not supported");
                        }
                        Some(Ok(Message::Close(_))) => {
                            info!("Connection {} closed by client", self.id);
                            break;
                        }
                        Some(Ok(Message::Pong(_))) => {
                            if let Some(mut conn) = self.manager.get_connection_mut(&self.id) {
                                conn.update_ping();
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            if let Err(e) = self.ws_stream.send(Message::Pong(data)).await {
                                error!("Failed to send pong: {}", e);
                                break;
                            }
                        }
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                        None => {
                            info!("Connection {} stream ended", self.id);
                            break;
                        }
                        _ => {}
                    }
                }
                
                Some(server_msg) = self.rx.recv() => {
                    if let Err(e) = self.send_message(server_msg).await {
                        error!("Failed to send message: {}", e);
                        break;
                    }
                }
                
                _ = heartbeat.tick() => {
                    if let Err(e) = self.ws_stream.send(Message::Ping(vec![])).await {
                        error!("Failed to send ping: {}", e);
                        break;
                    }
                }
            }
        }
        
        // Cleanup on disconnect
        self.handle_disconnect().await;
        info!("Connection {} handler ended", self.id);
    }

    async fn handle_text_message(&mut self, text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message: ClientMessage = serde_json::from_str(&text)?;
        debug!("Received message from {}: {:?}", self.id, message);
        
        // Update ping time
        if let Some(mut conn) = self.manager.get_connection_mut(&self.id) {
            conn.update_ping();
        }
        
        match message {
            ClientMessage::Ping => {
                self.send_message(ServerMessage::Pong).await?;
            }
            ClientMessage::JoinQueue { player_name } => {
                self.handle_join_queue(player_name).await?;
            }
            ClientMessage::Reconnect { session_token } => {
                self.handle_reconnect(session_token).await?;
            }
            ClientMessage::PlayCard { card } => {
                self.handle_play_card(card).await?;
            }
            ClientMessage::RequestState => {
                self.handle_request_state().await?;
            }
        }
        
        Ok(())
    }

    async fn send_message(&mut self, message: ServerMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let text = serde_json::to_string(&message)?;
        self.ws_stream.send(Message::Text(text)).await?;
        Ok(())
    }

    async fn handle_join_queue(&mut self, player_name: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Update connection with player name
        if let Some(mut conn) = self.manager.get_connection_mut(&self.id) {
            conn.player_name = Some(player_name.clone());
        }
        
        // Try to join or create a game
        match self.sessions.join_or_create_game(self.id, player_name).await {
            Ok((session_id, player_id, session_token)) => {
                // Update connection with session info
                if let Some(mut conn) = self.manager.get_connection_mut(&self.id) {
                    conn.session_id = Some(session_id);
                }
                
                // Send connection confirmation
                self.send_message(ServerMessage::Connected {
                    player_id,
                    session_token,
                }).await?;
                
                // Check if game is ready to start
                if let Some(game_start_info) = self.sessions.check_game_ready(session_id).await {
                    // Game is starting - send start messages to both players
                    for (conn_id, msg) in game_start_info {
                        if let Some(conn) = self.manager.get_connection(&conn_id) {
                            conn.send(msg).await.ok();
                        }
                    }
                } else {
                    // Still waiting for opponent
                    self.send_message(ServerMessage::WaitingForOpponent).await?;
                }
            }
            Err(e) => {
                self.send_message(ServerMessage::Error {
                    message: format!("Failed to join game: {}", e),
                }).await?;
            }
        }
        
        Ok(())
    }

    async fn handle_reconnect(&mut self, session_token: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.sessions.reconnect_player(self.id, session_token).await {
            Ok((session_id, player_id, messages)) => {
                // Update connection with session info
                if let Some(mut conn) = self.manager.get_connection_mut(&self.id) {
                    conn.session_id = Some(session_id);
                }
                
                // Send reconnection messages
                for msg in messages {
                    self.send_message(msg).await?;
                }
                
                // Notify opponent of reconnection
                if let Some(opponent_id) = self.sessions.get_opponent_connection(session_id, player_id).await {
                    if let Some(conn) = self.manager.get_connection(&opponent_id) {
                        conn.send(ServerMessage::OpponentReconnected).await.ok();
                    }
                }
            }
            Err(e) => {
                self.send_message(ServerMessage::Error {
                    message: format!("Failed to reconnect: {}", e),
                }).await?;
            }
        }
        
        Ok(())
    }

    async fn handle_play_card(&mut self, card: crate::components::card::Card) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.manager.get_connection(&self.id)
            .ok_or("Connection not found")?;
        
        let session_id = conn.session_id
            .ok_or("Not in a game session")?;
        
        drop(conn); // Release the lock
        
        match self.sessions.play_card(session_id, self.id, card).await {
            Ok(messages) => {
                // Send messages to relevant players
                for (conn_id, msg) in messages {
                    if let Some(conn) = self.manager.get_connection(&conn_id) {
                        conn.send(msg).await.ok();
                    }
                }
            }
            Err(e) => {
                self.send_message(ServerMessage::InvalidMove {
                    reason: e.to_string(),
                }).await?;
            }
        }
        
        Ok(())
    }

    async fn handle_request_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.manager.get_connection(&self.id)
            .ok_or("Connection not found")?;
        
        let session_id = conn.session_id
            .ok_or("Not in a game session")?;
        
        drop(conn); // Release the lock
        
        if let Some(state_msg) = self.sessions.get_state_for_player(session_id, self.id).await {
            self.send_message(state_msg).await?;
        } else {
            self.send_message(ServerMessage::Error {
                message: "Game session not found".to_string(),
            }).await?;
        }
        
        Ok(())
    }

    async fn handle_disconnect(&mut self) {
        // Remove from connection manager
        if let Some(conn) = self.manager.remove_connection(&self.id) {
            if let Some(session_id) = conn.session_id {
                // Notify session manager of disconnection
                if let Some((opponent_id, timeout_msg)) = self.sessions.handle_player_disconnect(session_id, self.id).await {
                    // Notify opponent
                    if let Some(opp_conn) = self.manager.get_connection(&opponent_id) {
                        opp_conn.send(timeout_msg).await.ok();
                    }
                }
            }
        }
    }
}