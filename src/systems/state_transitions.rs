use bevy::prelude::*;
use crate::systems::game_logic::*;
use crate::components::*;

pub fn setup_game_system(
    mut commands: Commands,
    mut game_manager: ResMut<GameManager>,
    mut turn_manager: ResMut<TurnManager>,
    mut round_state: ResMut<RoundState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // info!("Setting up new game");
    
    // Reset all game state
    *game_manager = GameManager::new(6); // Standard Kseri has 6 rounds
    *turn_manager = TurnManager::default();
    *round_state = RoundState::default();
    
    // Create deck entity
    let deck_entity = commands.spawn((
        DeckComponent,
        Deck::new(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    )).id();
    
    // Create table entity
    let table_entity = commands.spawn((
        TableComponent,
        TablePile::new(),
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
    )).id();
    
    // Create player entities
    let _player_one = commands.spawn((
        Player {
            id: PlayerId::PLAYER_ONE,
            name: "Player 1".to_string(),
            is_local: true,
        },
        Hand::new(4),
        Score::new(),
        Transform::from_xyz(-5.0, -3.0, 0.0),
        GlobalTransform::default(),
    )).id();
    
    let _player_two = commands.spawn((
        Player {
            id: PlayerId::PLAYER_TWO,
            name: "Player 2".to_string(),
            is_local: false,
        },
        Hand::new(4),
        Score::new(),
        Transform::from_xyz(5.0, -3.0, 0.0),
        GlobalTransform::default(),
    )).id();
    
    game_manager.deck_entity = Some(deck_entity);
    game_manager.table_entity = Some(table_entity);
    
    // Transition to playing state
    next_state.set(GameState::Playing);
}

pub fn check_game_state_transitions(
    game_state: Res<State<GameState>>,
    playing_phase: Option<Res<State<PlayingPhase>>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    next_playing_phase: Option<ResMut<NextState<PlayingPhase>>>,
    game_manager: Res<GameManager>,
    deck_query: Query<&Deck, With<DeckComponent>>,
    hand_query: Query<&Hand, With<Player>>,
) {
    match game_state.get() {
        GameState::GameSetup => {
            // Setup is handled by setup_game_system
        }
        GameState::Playing => {
            if let (Some(phase), Some(mut next_phase)) = (playing_phase, next_playing_phase) {
                match phase.get() {
                    PlayingPhase::DealingCards => {
                        // Check if dealing is complete
                        let all_hands_ready = hand_query.iter().all(|hand| hand.count() == 4);
                        if all_hands_ready {
                            next_phase.set(PlayingPhase::PlayerTurn);
                        }
                    }
                    PlayingPhase::PlayerTurn => {
                        // Check for round end conditions
                        let hands_empty = hand_query.iter().all(|hand| hand.is_empty());
                        let deck_empty = deck_query.single()
                            .map(|deck| deck.remaining() == 0)
                            .unwrap_or(true);
                        
                        if hands_empty && deck_empty {
                            next_phase.set(PlayingPhase::RoundEnd);
                        } else if hands_empty && !deck_empty {
                            next_phase.set(PlayingPhase::DealingCards);
                        }
                    }
                    PlayingPhase::ProcessingCapture => {
                        // This phase is temporary, automatically transition back
                        next_phase.set(PlayingPhase::PlayerTurn);
                    }
                    PlayingPhase::RoundEnd => {
                        // Check if game is complete
                        if game_manager.is_game_complete() {
                            next_game_state.set(GameState::GameOver);
                        } else {
                            // Start new round
                            next_phase.set(PlayingPhase::DealingCards);
                        }
                    }
                }
            }
        }
        GameState::GameOver => {
            // Game over state is terminal until restart
        }
        _ => {}
    }
}

pub fn handle_round_end(
    mut commands: Commands,
    mut game_manager: ResMut<GameManager>,
    mut turn_manager: ResMut<TurnManager>,
    mut round_state: ResMut<RoundState>,
    mut round_end_events: EventReader<RoundEndEvent>,
    table_query: Query<(Entity, &TablePile), With<TableComponent>>,
    mut score_query: Query<(&Player, &mut Score)>,
    card_entities: Query<Entity, With<CardEntity>>,
) {
    for event in round_end_events.read() {
        // info!("Round {} ending: {:?}", game_manager.round_number, event.reason);
        
        // Award remaining table cards to last capturer
        if let Some(last_capture) = &game_manager.last_capture {
            if let Ok((_, table)) = table_query.single() {
                if !table.cards.is_empty() {
                    // Find the score component for the last capturer
                    for (player, mut score) in score_query.iter_mut() {
                        if player.id == last_capture.player_id {
                            // Table stores Entity references, add them to score
                            score.add_collected_cards(table.cards.clone());
                            // info!("Awarded {} remaining table cards to {:?}", 
                            //     table.cards.len(), player.id);
                            break;
                        }
                    }
                }
            }
        }
        
        // Clean up all card entities
        for entity in card_entities.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset for new round
        game_manager.start_new_round();
        turn_manager.reset_for_new_round();
        *round_state = RoundState::default();
        
        // Reset deck and shuffle
        if let Some(deck_entity) = game_manager.deck_entity {
            commands.entity(deck_entity).insert(Deck::new());
        }
    }
}

pub fn handle_state_transition_events(
    mut events: EventReader<GameStateTransitionEvent>,
) {
    for event in events.read() {
        // info!("Game state transition: {:?} -> {:?}", event.from, event.to);
    }
}