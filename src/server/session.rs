use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::components::card::{Card, PlayerId};
use crate::server::messages::{ServerMessage, GameEvent, GameEventType};
use crate::server::game_engine::KseriGameState;

const SESSION_TIMEOUT: Duration = Duration::from_secs(1800); // 30 minutes
const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const WAITING_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

#[derive(Debug, Clone)]
pub struct Player {
    pub connection_id: Uuid,
    pub name: String,
    pub player_id: PlayerId,
    pub session_token: String,
    pub connected: bool,
    pub disconnected_at: Option<Instant>,
}

pub struct GameSession {
    pub id: Uuid,
    pub players: [Option<Player>; 2],
    pub game_state: RwLock<KseriGameState>,
    pub created_at: Instant,
    pub last_activity: RwLock<Instant>,
    pub event_history: RwLock<VecDeque<GameEvent>>,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            players: [None, None],
            game_state: RwLock::new(KseriGameState::new()),
            created_at: Instant::now(),
            last_activity: RwLock::new(Instant::now()),
            event_history: RwLock::new(VecDeque::with_capacity(50)),
        }
    }
    
    pub fn add_player(&mut self, connection_id: Uuid, name: String) -> Result<(PlayerId, String), &'static str> {
        // Find empty slot
        let slot = if self.players[0].is_none() {
            0
        } else if self.players[1].is_none() {
            1
        } else {
            return Err("Game session is full");
        };
        
        let player_id = if slot == 0 { PlayerId::PLAYER_ONE } else { PlayerId::PLAYER_TWO };
        let session_token = format!("{}-{}", self.id, Uuid::new_v4());
        
        self.players[slot] = Some(Player {
            connection_id,
            name: name.clone(),
            player_id,
            session_token: session_token.clone(),
            connected: true,
            disconnected_at: None,
        });
        
        Ok((player_id, session_token))
    }
    
    pub fn is_ready(&self) -> bool {
        self.players[0].is_some() && self.players[1].is_some()
    }
    
    pub fn get_player_by_connection(&self, connection_id: Uuid) -> Option<&Player> {
        self.players.iter()
            .filter_map(|p| p.as_ref())
            .find(|p| p.connection_id == connection_id)
    }
    
    pub fn get_player_by_token(&self, token: &str) -> Option<&Player> {
        self.players.iter()
            .filter_map(|p| p.as_ref())
            .find(|p| p.session_token == token)
    }
    
    pub fn get_opponent(&self, player_id: PlayerId) -> Option<&Player> {
        let opponent_idx = if player_id == PlayerId::PLAYER_ONE { 1 } else { 0 };
        self.players[opponent_idx].as_ref()
    }
    
    pub async fn add_event(&self, event_type: GameEventType) {
        let event = GameEvent {
            session_id: self.id,
            event_type,
            timestamp: Instant::now(),
        };
        
        let mut history = self.event_history.write().await;
        history.push_back(event);
        if history.len() > 50 {
            history.pop_front();
        }
        
        *self.last_activity.write().await = Instant::now();
    }
    
    pub fn is_expired(&self) -> bool {
        // Check if all players are disconnected
        let all_disconnected = self.players.iter()
            .filter_map(|p| p.as_ref())
            .all(|p| !p.connected);
            
        if all_disconnected {
            // Check disconnection timeout
            let max_disconnect_time = self.players.iter()
                .filter_map(|p| p.as_ref())
                .filter_map(|p| p.disconnected_at)
                .max();
                
            if let Some(disconnect_time) = max_disconnect_time {
                return disconnect_time.elapsed() > RECONNECT_TIMEOUT;
            }
        }
        
        // Check session timeout
        self.created_at.elapsed() > SESSION_TIMEOUT
    }
}

pub struct WaitingPlayer {
    pub connection_id: Uuid,
    pub name: String,
    pub joined_at: Instant,
}

pub struct SessionManager {
    pub sessions: Arc<DashMap<Uuid, GameSession>>,
    waiting_queue: Arc<RwLock<VecDeque<WaitingPlayer>>>,
    connection_to_session: Arc<DashMap<Uuid, Uuid>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            waiting_queue: Arc::new(RwLock::new(VecDeque::new())),
            connection_to_session: Arc::new(DashMap::new()),
        }
    }
    
    pub async fn join_or_create_game(
        &self,
        connection_id: Uuid,
        player_name: String,
    ) -> Result<(Uuid, PlayerId, String), Box<dyn std::error::Error + Send + Sync>> {
        // Check if player is already in a session
        if let Some(_session_id) = self.connection_to_session.get(&connection_id) {
            return Err("Already in a game session".into());
        }
        
        let mut queue = self.waiting_queue.write().await;
        
        // Check if there's a waiting player
        if let Some(waiting) = queue.pop_front() {
            // Create new game session
            let mut session = GameSession::new();
            let session_id = session.id;
            
            // Add both players
            let (_p1_id, _p1_token) = session.add_player(waiting.connection_id, waiting.name)?;
            let (p2_id, p2_token) = session.add_player(connection_id, player_name.clone())?;
            
            // Store session
            self.sessions.insert(session_id, session);
            self.connection_to_session.insert(waiting.connection_id, session_id);
            self.connection_to_session.insert(connection_id, session_id);
            
            info!("Created game session {} with two players", session_id);
            
            // Return info for the current player
            Ok((session_id, p2_id, p2_token))
        } else {
            // Add to waiting queue
            queue.push_back(WaitingPlayer {
                connection_id,
                name: player_name.clone(),
                joined_at: Instant::now(),
            });
            
            info!("Player {} added to waiting queue", connection_id);
            
            // Create placeholder session for waiting player
            let mut session = GameSession::new();
            let session_id = session.id;
            let (player_id, token) = session.add_player(connection_id, player_name)?;
            
            self.sessions.insert(session_id, session);
            self.connection_to_session.insert(connection_id, session_id);
            
            Ok((session_id, player_id, token))
        }
    }
    
    pub async fn check_game_ready(&self, session_id: Uuid) -> Option<Vec<(Uuid, ServerMessage)>> {
        let session = self.sessions.get(&session_id)?;
        
        if !session.is_ready() {
            return None;
        }
        
        let mut messages = Vec::new();
        let game_state = session.game_state.read().await;
        
        // Create start messages for both players
        for (idx, player) in session.players.iter().enumerate() {
            if let Some(p) = player {
                let opponent = session.players[1 - idx].as_ref()?;
                
                let msg = ServerMessage::GameStarted {
                    opponent_name: opponent.name.clone(),
                    your_turn: p.player_id == game_state.current_turn,
                    initial_hand: game_state.player_hands[idx].clone(),
                    table_cards: game_state.table_cards.clone(),
                };
                
                messages.push((p.connection_id, msg));
            }
        }
        
        drop(game_state);
        session.add_event(GameEventType::PlayerJoined(PlayerId::PLAYER_TWO, session.players[1].as_ref()?.name.clone())).await;
        
        Some(messages)
    }
    
    pub async fn reconnect_player(
        &self,
        connection_id: Uuid,
        session_token: String,
    ) -> Result<(Uuid, PlayerId, Vec<ServerMessage>), Box<dyn std::error::Error + Send + Sync>> {
        // Find session by token and get the session ID
        let session_id = {
            let session_entry = self.sessions.iter()
                .find(|entry| {
                    entry.value().players.iter()
                        .filter_map(|p| p.as_ref())
                        .any(|p| p.session_token == session_token)
                })
                .ok_or("Invalid session token")?;
            *session_entry.key()
        };
        
        // Get mutable session reference
        let mut session_entry = self.sessions.get_mut(&session_id)
            .ok_or("Session not found")?;
        let session = session_entry.value_mut();
        
        // Find and update player
        let player_idx = session.players.iter()
            .position(|p| p.as_ref().map(|p| p.session_token == session_token).unwrap_or(false))
            .ok_or("Player not found in session")?;
            
        let player = session.players[player_idx].as_mut()
            .ok_or("Player slot is empty")?;
            
        // Check reconnection timeout
        if let Some(disconnect_time) = player.disconnected_at {
            if disconnect_time.elapsed() > RECONNECT_TIMEOUT {
                return Err("Reconnection timeout expired".into());
            }
        }
        
        // Update player connection
        player.connection_id = connection_id;
        player.connected = true;
        player.disconnected_at = None;
        
        let player_id = player.player_id;
        
        // Drop the mutable reference to session
        drop(session_entry);
        
        // Update mappings
        self.connection_to_session.insert(connection_id, session_id);
        
        // Prepare reconnection messages
        let mut messages = vec![
            ServerMessage::Connected {
                player_id,
                session_token: session_token.clone(),
            }
        ];
        
        // Get session again for reading
        let session = self.sessions.get(&session_id)
            .ok_or("Session not found")?;
        
        // Send current game state
        let game_state = session.game_state.read().await;
        let state_msg = self.create_state_message(&game_state, player_id, &session);
        messages.push(state_msg);
        drop(game_state);
        
        // Add reconnection event
        session.add_event(GameEventType::PlayerReconnected(player_id)).await;
        
        Ok((session_id, player_id, messages))
    }
    
    pub async fn play_card(
        &self,
        session_id: Uuid,
        connection_id: Uuid,
        card: Card,
    ) -> Result<Vec<(Uuid, ServerMessage)>, Box<dyn std::error::Error + Send + Sync>> {
        let session = self.sessions.get(&session_id)
            .ok_or("Session not found")?;
            
        // Verify player
        let player = session.get_player_by_connection(connection_id)
            .ok_or("Player not found in session")?;
            
        let player_id = player.player_id;
        
        // Play the card
        let mut game_state = session.game_state.write().await;
        let result = game_state.play_card(player_id, card)?;
        
        // Create messages for both players
        let mut messages = Vec::new();
        
        // Card played message
        let card_played_msg = ServerMessage::CardPlayed {
            player: player_id,
            card,
            captured_cards: result.captured_cards.clone(),
            is_kseri: result.is_kseri,
        };
        
        // Send to both players
        for p in session.players.iter().filter_map(|p| p.as_ref()) {
            if p.connected {
                messages.push((p.connection_id, card_played_msg.clone()));
            }
        }
        
        // Check if need to deal new cards
        if game_state.player_hands[0].is_empty() && game_state.player_hands[1].is_empty() && !game_state.deck.is_empty() {
            // Deal was already done in play_card, just notify players
            for (idx, p) in session.players.iter().enumerate() {
                if let Some(player) = p {
                    if player.connected {
                        let new_cards_msg = ServerMessage::NewCards {
                            cards: game_state.player_hands[idx].clone(),
                        };
                        messages.push((player.connection_id, new_cards_msg));
                    }
                }
            }
        }
        
        // Send state updates
        for p in session.players.iter().filter_map(|p| p.as_ref()) {
            if p.connected {
                let state_msg = self.create_state_message(&game_state, p.player_id, &session);
                messages.push((p.connection_id, state_msg));
            }
        }
        
        // Check game over
        if result.game_ended {
            let winner = game_state.get_winner();
            let scores = game_state.calculate_scores();
            
            let game_over_msg = ServerMessage::GameOver {
                winner,
                final_scores: scores,
                captured_cards: [
                    game_state.player_scores[0].clone(),
                    game_state.player_scores[1].clone(),
                ],
            };
            
            for p in session.players.iter().filter_map(|p| p.as_ref()) {
                if p.connected {
                    messages.push((p.connection_id, game_over_msg.clone()));
                }
            }
            
            drop(game_state);
            session.add_event(GameEventType::GameEnded(winner)).await;
        } else {
            drop(game_state);
            session.add_event(GameEventType::CardPlayed(player_id, card)).await;
        }
        
        Ok(messages)
    }
    
    pub async fn get_state_for_player(&self, session_id: Uuid, connection_id: Uuid) -> Option<ServerMessage> {
        let session = self.sessions.get(&session_id)?;
        let player = session.get_player_by_connection(connection_id)?;
        let game_state = session.game_state.read().await;
        
        Some(self.create_state_message(&game_state, player.player_id, &session))
    }
    
    fn create_state_message(&self, game_state: &KseriGameState, player_id: PlayerId, _session: &GameSession) -> ServerMessage {
        let player_idx = player_id.0 as usize;
        let opponent_idx = 1 - player_idx;
        let scores = game_state.calculate_scores();
        
        ServerMessage::StateUpdate {
            hand_count: game_state.player_hands[player_idx].len() as u8,
            opponent_hand_count: game_state.player_hands[opponent_idx].len() as u8,
            table_cards: game_state.table_cards.clone(),
            your_score: scores[player_idx],
            opponent_score: scores[opponent_idx],
            deck_remaining: game_state.deck.len() as u8,
            your_turn: game_state.current_turn == player_id,
            last_capture_player: game_state.last_capture_player,
        }
    }
    
    pub async fn handle_player_disconnect(&self, session_id: Uuid, connection_id: Uuid) -> Option<(Uuid, ServerMessage)> {
        let mut session = self.sessions.get_mut(&session_id)?;
        
        // Find and update player
        let player_idx = session.players.iter()
            .position(|p| p.as_ref().map(|p| p.connection_id == connection_id).unwrap_or(false))?;
            
        if let Some(player) = &mut session.players[player_idx] {
            player.connected = false;
            player.disconnected_at = Some(Instant::now());
            
            let player_id = player.player_id;
            
            // Remove from connection mapping
            self.connection_to_session.remove(&connection_id);
            
            // Notify opponent
            if let Some(opponent) = session.get_opponent(player_id) {
                if opponent.connected {
                    let msg = ServerMessage::OpponentDisconnected {
                        timeout_seconds: RECONNECT_TIMEOUT.as_secs() as u32,
                    };
                    
                    // Add event asynchronously
                    let session_id = session.id;
                    let sessions = self.sessions.clone();
                    tokio::spawn(async move {
                        if let Some(s) = sessions.get(&session_id) {
                            s.add_event(GameEventType::PlayerDisconnected(player_id)).await;
                        }
                    });
                    
                    return Some((opponent.connection_id, msg));
                }
            }
        }
        
        None
    }
    
    pub async fn get_opponent_connection(&self, session_id: Uuid, player_id: PlayerId) -> Option<Uuid> {
        let session = self.sessions.get(&session_id)?;
        session.get_opponent(player_id)
            .filter(|p| p.connected)
            .map(|p| p.connection_id)
    }
    
    pub async fn cleanup_expired_sessions(&self) {
        let expired: Vec<Uuid> = self.sessions.iter()
            .filter(|entry| entry.value().is_expired())
            .map(|entry| *entry.key())
            .collect();
            
        for session_id in expired {
            if let Some((_, session)) = self.sessions.remove(&session_id) {
                // Remove connection mappings
                for player in session.players.iter().filter_map(|p| p.as_ref()) {
                    self.connection_to_session.remove(&player.connection_id);
                }
                
                info!("Cleaned up expired session {}", session_id);
            }
        }
        
        // Clean up waiting queue
        let mut queue = self.waiting_queue.write().await;
        queue.retain(|player| player.joined_at.elapsed() < WAITING_TIMEOUT);
    }
}