use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{interval, Instant};
use tracing::{debug, error};
use uuid::Uuid;

use crate::server::messages::ServerMessage;

const BATCH_INTERVAL: Duration = Duration::from_millis(10);
const MAX_BATCH_SIZE: usize = 50;

#[derive(Debug, Clone)]
pub struct BroadcastMessage {
    pub target: BroadcastTarget,
    pub message: ServerMessage,
    pub priority: MessagePriority,
}

#[derive(Debug, Clone)]
pub enum BroadcastTarget {
    Single(Uuid),
    Multiple(Vec<Uuid>),
    Session(Uuid),
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

pub struct MessageBatch {
    messages: Vec<BroadcastMessage>,
    created_at: Instant,
}

impl MessageBatch {
    fn new() -> Self {
        Self {
            messages: Vec::with_capacity(MAX_BATCH_SIZE),
            created_at: Instant::now(),
        }
    }
    
    fn add(&mut self, msg: BroadcastMessage) -> bool {
        if self.messages.len() >= MAX_BATCH_SIZE {
            return false;
        }
        self.messages.push(msg);
        true
    }
    
    fn should_send(&self) -> bool {
        !self.messages.is_empty() && 
        (self.messages.len() >= MAX_BATCH_SIZE || 
         self.created_at.elapsed() >= BATCH_INTERVAL ||
         self.messages.iter().any(|m| m.priority >= MessagePriority::High))
    }
    
    fn take_messages(self) -> Vec<BroadcastMessage> {
        self.messages
    }
}

pub struct Broadcaster {
    tx: mpsc::UnboundedSender<BroadcastMessage>,
    _handle: tokio::task::JoinHandle<()>,
}

impl Broadcaster {
    pub fn new(
        connection_manager: Arc<crate::server::connection::ConnectionManager>,
        session_manager: Arc<crate::server::session::SessionManager>,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        let handle = tokio::spawn(async move {
            Self::run_broadcaster(rx, connection_manager, session_manager).await;
        });
        
        Self {
            tx,
            _handle: handle,
        }
    }
    
    pub fn send(&self, target: BroadcastTarget, message: ServerMessage, priority: MessagePriority) {
        let msg = BroadcastMessage {
            target,
            message,
            priority,
        };
        
        if let Err(e) = self.tx.send(msg) {
            error!("Failed to queue broadcast message: {}", e);
        }
    }
    
    pub fn send_to_player(&self, player_id: Uuid, message: ServerMessage) {
        self.send(BroadcastTarget::Single(player_id), message, MessagePriority::Normal);
    }
    
    pub fn send_to_session(&self, session_id: Uuid, message: ServerMessage) {
        self.send(BroadcastTarget::Session(session_id), message, MessagePriority::Normal);
    }
    
    pub fn send_critical(&self, target: BroadcastTarget, message: ServerMessage) {
        self.send(target, message, MessagePriority::Critical);
    }
    
    async fn run_broadcaster(
        mut rx: mpsc::UnboundedReceiver<BroadcastMessage>,
        connection_manager: Arc<crate::server::connection::ConnectionManager>,
        session_manager: Arc<crate::server::session::SessionManager>,
    ) {
        let mut batch = MessageBatch::new();
        let mut timer = interval(BATCH_INTERVAL);
        
        loop {
            tokio::select! {
                Some(msg) = rx.recv() => {
                    let priority = msg.priority;
                    
                    if !batch.add(msg) || priority >= MessagePriority::High {
                        // Batch is full or high priority - send immediately
                        let messages = batch.take_messages();
                        batch = MessageBatch::new();
                        Self::process_batch(messages, &connection_manager, &session_manager).await;
                        
                        // Add the message to new batch if it wasn't added
                        if priority < MessagePriority::High {
                            if let Some(msg) = rx.try_recv().ok() {
                                batch.add(msg);
                            }
                        }
                    }
                }
                
                _ = timer.tick() => {
                    if batch.should_send() {
                        let messages = batch.take_messages();
                        batch = MessageBatch::new();
                        Self::process_batch(messages, &connection_manager, &session_manager).await;
                    }
                }
                
                else => {
                    debug!("Broadcaster shutting down");
                    break;
                }
            }
        }
    }
    
    async fn process_batch(
        messages: Vec<BroadcastMessage>,
        connection_manager: &Arc<crate::server::connection::ConnectionManager>,
        session_manager: &Arc<crate::server::session::SessionManager>,
    ) {
        // Group messages by target for efficiency
        let mut by_connection: std::collections::HashMap<Uuid, Vec<ServerMessage>> = std::collections::HashMap::new();
        
        for msg in messages {
            match msg.target {
                BroadcastTarget::Single(conn_id) => {
                    by_connection.entry(conn_id)
                        .or_insert_with(Vec::new)
                        .push(msg.message);
                }
                
                BroadcastTarget::Multiple(conn_ids) => {
                    for conn_id in conn_ids {
                        by_connection.entry(conn_id)
                            .or_insert_with(Vec::new)
                            .push(msg.message.clone());
                    }
                }
                
                BroadcastTarget::Session(session_id) => {
                    // Get all connections in session
                    if let Some(session) = session_manager.sessions.get(&session_id) {
                        for player in session.players.iter().filter_map(|p| p.as_ref()) {
                            if player.connected {
                                by_connection.entry(player.connection_id)
                                    .or_insert_with(Vec::new)
                                    .push(msg.message.clone());
                            }
                        }
                    }
                }
                
                BroadcastTarget::All => {
                    // This would be expensive - only use for server-wide announcements
                    // For now, we'll skip implementation
                    debug!("Broadcast to all not implemented");
                }
            }
        }
        
        // Send messages to connections
        for (conn_id, messages) in by_connection {
            if let Some(conn) = connection_manager.get_connection(&conn_id) {
                for msg in messages {
                    if let Err(e) = conn.send(msg).await {
                        debug!("Failed to send message to {}: {}", conn_id, e);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_batch() {
        let mut batch = MessageBatch::new();
        
        let msg = BroadcastMessage {
            target: BroadcastTarget::Single(Uuid::new_v4()),
            message: ServerMessage::Pong,
            priority: MessagePriority::Normal,
        };
        
        assert!(batch.add(msg.clone()));
        assert_eq!(batch.messages.len(), 1);
        
        // Fill batch
        for _ in 1..MAX_BATCH_SIZE {
            assert!(batch.add(msg.clone()));
        }
        
        // Batch should be full
        assert!(!batch.add(msg.clone()));
        assert_eq!(batch.messages.len(), MAX_BATCH_SIZE);
    }
    
    #[test]
    fn test_priority_ordering() {
        let low = MessagePriority::Low;
        let normal = MessagePriority::Normal;
        let high = MessagePriority::High;
        let critical = MessagePriority::Critical;
        
        assert!(low < normal);
        assert!(normal < high);
        assert!(high < critical);
    }
}