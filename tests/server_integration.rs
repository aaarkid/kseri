use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::json;

#[tokio::test]
async fn test_two_players_connect_and_start_game() {
    // Give server time to start if running in same test suite
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    let url = "ws://localhost:8080";
    
    // Connect first player
    let (ws_stream1, _) = connect_async(url).await.expect("Failed to connect player 1");
    let (mut write1, mut read1) = ws_stream1.split();
    
    // Send join queue for player 1
    let join_msg1 = json!({
        "type": "JoinQueue",
        "data": {
            "player_name": "Player1"
        }
    });
    write1.send(Message::Text(join_msg1.to_string())).await.unwrap();
    
    // Read player 1 messages
    let player1_task = tokio::spawn(async move {
        let mut messages = Vec::new();
        while let Some(msg) = read1.next().await {
            if let Ok(Message::Text(text)) = msg {
                messages.push(text);
                if messages.len() >= 2 { // Connected + WaitingForOpponent
                    break;
                }
            }
        }
        messages
    });
    
    // Give time for first player to register
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Connect second player
    let (ws_stream2, _) = connect_async(url).await.expect("Failed to connect player 2");
    let (mut write2, mut read2) = ws_stream2.split();
    
    // Send join queue for player 2
    let join_msg2 = json!({
        "type": "JoinQueue",
        "data": {
            "player_name": "Player2"
        }
    });
    write2.send(Message::Text(join_msg2.to_string())).await.unwrap();
    
    // Read player 2 messages
    let player2_task = tokio::spawn(async move {
        let mut messages = Vec::new();
        while let Some(msg) = read2.next().await {
            if let Ok(Message::Text(text)) = msg {
                messages.push(text);
                if messages.len() >= 2 { // Connected + GameStarted
                    break;
                }
            }
        }
        messages
    });
    
    // Wait for both players to receive messages
    let player1_messages = player1_task.await.unwrap();
    let player2_messages = player2_task.await.unwrap();
    
    // Verify player 1 received Connected and WaitingForOpponent
    assert!(player1_messages.len() >= 2);
    assert!(player1_messages[0].contains("Connected"));
    assert!(player1_messages[1].contains("WaitingForOpponent"));
    
    // Verify player 2 received Connected and GameStarted
    assert!(player2_messages.len() >= 2);
    assert!(player2_messages[0].contains("Connected"));
    assert!(player2_messages[1].contains("GameStarted"));
    
    println!("Test passed! Two players connected and game started.");
}