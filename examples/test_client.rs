use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ClientMessage {
    JoinQueue { player_name: String },
    PlayCard { card: Card },
    RequestState,
    Ping,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://localhost:8080";
    
    println!("Connecting to {}...", url);
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected!");
    
    let (mut write, mut read) = ws_stream.split();
    
    // Send join queue message
    let join_msg = ClientMessage::JoinQueue {
        player_name: "TestPlayer".to_string(),
    };
    let json = serde_json::to_string(&join_msg)?;
    write.send(Message::Text(json)).await?;
    println!("Sent join queue message");
    
    // Read messages
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received: {}", text);
                }
                Ok(Message::Binary(_)) => {
                    println!("Received binary message");
                }
                Ok(Message::Close(_)) => {
                    println!("Server closed connection");
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Keep connection alive
    println!("Press Ctrl+C to exit");
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}