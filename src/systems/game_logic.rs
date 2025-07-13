use bevy::prelude::*;
use crate::components::{Card, PlayerId};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Connecting,
    Playing,
    GameOver,
    GameSetup,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayingPhase {
    #[default]
    DealingCards,
    PlayerTurn,
    ProcessingCapture,
    RoundEnd,
}

#[derive(Resource)]
pub struct GameManager {
    pub round_number: u32,
    pub total_rounds: u32,
    pub deck_entity: Option<Entity>,
    pub table_entity: Option<Entity>,
    pub last_capture: Option<LastCapture>,
}

impl GameManager {
    pub fn new(total_rounds: u32) -> Self {
        Self {
            round_number: 0,
            total_rounds,
            deck_entity: None,
            table_entity: None,
            last_capture: None,
        }
    }

    pub fn start_new_round(&mut self) {
        self.round_number += 1;
    }

    pub fn is_game_complete(&self) -> bool {
        self.round_number >= self.total_rounds
    }
}

#[derive(Resource, Default)]
pub struct TurnManager {
    pub current_player: PlayerId,
    pub turn_number: u32,
    pub waiting_for_action: bool,
    pub actions_this_turn: u32,
}

impl TurnManager {
    pub fn switch_turn(&mut self) {
        self.current_player = match self.current_player {
            PlayerId::PLAYER_ONE => PlayerId::PLAYER_TWO,
            PlayerId::PLAYER_TWO => PlayerId::PLAYER_ONE,
            _ => PlayerId::PLAYER_ONE,
        };
        self.turn_number += 1;
        self.waiting_for_action = false;
        self.actions_this_turn = 0;
    }
    
    pub fn reset_for_new_round(&mut self) {
        self.turn_number = 0;
        self.actions_this_turn = 0;
        self.waiting_for_action = false;
        // Current player stays the same for the new round
    }
}

#[derive(Resource, Default)]
pub struct RoundState {
    pub cards_dealt: u32,
    pub initial_table_cards: Vec<Card>,
    pub captures_this_round: Vec<(PlayerId, Vec<Card>)>,
}

#[derive(Clone)]
pub struct LastCapture {
    pub player_id: PlayerId,
    pub captured_cards: Vec<Card>,
    pub was_kseri: bool,
}

#[derive(Event)]
pub struct PlayerActionEvent {
    pub player_id: PlayerId,
    pub action: PlayerAction,
}

#[derive(Clone)]
pub enum PlayerAction {
    PlayCard(Entity),
}

#[derive(Event)]
pub struct CaptureEvent {
    pub player_id: PlayerId,
    pub played_card: Card,
    pub captured_cards: Vec<Card>,
    pub captured_entities: Vec<Entity>,
    pub is_kseri: bool,
    pub is_double_kseri: bool,
}

#[derive(Event)]
pub struct RoundEndEvent {
    pub round_number: u32,
    pub player_scores: [(PlayerId, u32); 2],
}

#[derive(Event)]
pub struct GameStateTransitionEvent {
    pub from: GameState,
    pub to: GameState,
}

#[derive(Event)]
pub struct KseriEvent {
    pub player_id: PlayerId,
    pub card: Card,
}

#[derive(Event)]
pub struct GameOverEvent {
    pub winner: Option<PlayerId>,
    pub final_scores: [(PlayerId, u32); 2],
}