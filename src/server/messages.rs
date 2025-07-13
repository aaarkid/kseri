use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::components::card::{Card, PlayerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinQueue { player_name: String },
    Reconnect { session_token: String },
    PlayCard { card: Card },
    RequestState,
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    Connected { 
        player_id: PlayerId, 
        session_token: String 
    },
    WaitingForOpponent,
    GameStarted { 
        opponent_name: String, 
        your_turn: bool,
        initial_hand: Vec<Card>,
        table_cards: Vec<Card>,
    },
    CardPlayed { 
        player: PlayerId, 
        card: Card,
        captured_cards: Vec<Card>,
        is_kseri: bool,
    },
    NewCards {
        cards: Vec<Card>,
    },
    StateUpdate {
        hand_count: u8,
        opponent_hand_count: u8,
        table_cards: Vec<Card>,
        your_score: u32,
        opponent_score: u32,
        deck_remaining: u8,
        your_turn: bool,
        last_capture_player: Option<PlayerId>,
    },
    InvalidMove { 
        reason: String 
    },
    OpponentDisconnected {
        timeout_seconds: u32,
    },
    OpponentReconnected,
    GameOver { 
        winner: Option<PlayerId>, 
        final_scores: [u32; 2],
        captured_cards: [Vec<Card>; 2],
    },
    Error {
        message: String,
    },
    Pong,
}

#[derive(Debug, Clone)]
pub struct GameEvent {
    pub session_id: Uuid,
    pub event_type: GameEventType,
    pub timestamp: std::time::Instant,
}

#[derive(Debug, Clone)]
pub enum GameEventType {
    PlayerJoined(PlayerId, String),
    PlayerDisconnected(PlayerId),
    PlayerReconnected(PlayerId),
    CardPlayed(PlayerId, Card),
    CardsDealt,
    GameEnded(Option<PlayerId>),
}