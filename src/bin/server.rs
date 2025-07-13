use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Semaphore};
use tokio::time::interval;
use tokio_tungstenite::accept_async;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// Import from main crate
use kseri::server::{
    Connection, ConnectionHandler, ConnectionManager,
    SessionManager, Broadcaster,
};

const DEFAULT_PORT: u16 = 8080;
const MAX_CONNECTIONS: usize = 1000;
const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();
    
    // Get configuration from environment
    let port = env::var("KSERI_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);
    
    let max_connections = env::var("KSERI_MAX_CONNECTIONS")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(MAX_CONNECTIONS);
    
    // Create server components
    let connection_manager = Arc::new(ConnectionManager::new());
    let session_manager = Arc::new(SessionManager::new());
    let connection_semaphore = Arc::new(Semaphore::new(max_connections));
    
    // Create broadcaster (currently unused but ready for optimized broadcasting)
    let _broadcaster = Arc::new(Broadcaster::new(
        connection_manager.clone(),
        session_manager.clone(),
    ));
    
    // Bind to address
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = TcpListener::bind(addr).await?;
    
    info!("Kseri WebSocket server listening on {}", addr);
    info!("Max connections: {}", max_connections);
    
    // Spawn cleanup task
    let cleanup_connections = connection_manager.clone();
    let cleanup_sessions = session_manager.clone();
    tokio::spawn(async move {
        let mut interval = interval(CLEANUP_INTERVAL);
        loop {
            interval.tick().await;
            
            // Cleanup timed out connections
            let timed_out = cleanup_connections.cleanup_timed_out();
            if !timed_out.is_empty() {
                info!("Cleaned up {} timed out connections", timed_out.len());
            }
            
            // Cleanup expired sessions
            cleanup_sessions.cleanup_expired_sessions().await;
        }
    });
    
    // Main accept loop
    loop {
        let (stream, addr) = listener.accept().await?;
        
        // Acquire connection permit
        let permit = connection_semaphore.clone().acquire_owned().await?;
        
        let connections = connection_manager.clone();
        let sessions = session_manager.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, connections, sessions).await {
                error!("Error handling connection from {}: {}", addr, e);
            }
            drop(permit); // Release permit when connection ends
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    connection_manager: Arc<ConnectionManager>,
    session_manager: Arc<SessionManager>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("New TCP connection from {}", addr);
    
    // Accept WebSocket
    let ws_stream = accept_async(stream).await?;
    info!("WebSocket connection established from {}", addr);
    
    // Create connection
    let connection_id = Uuid::new_v4();
    let (tx, rx) = mpsc::unbounded_channel();
    let connection = Connection::new(connection_id, tx);
    
    // Add to manager
    connection_manager.add_connection(connection_id, connection);
    
    // Create and run handler
    let handler = ConnectionHandler::new(
        connection_id,
        ws_stream,
        rx,
        connection_manager,
        session_manager,
    );
    
    handler.run().await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_components_creation() {
        let connection_manager = Arc::new(ConnectionManager::new());
        let session_manager = Arc::new(SessionManager::new());
        let _broadcaster = Broadcaster::new(connection_manager.clone(), session_manager.clone());
        
        assert_eq!(connection_manager.get_connections_count(), 0);
    }
}