use bevy::prelude::*;
use crate::systems::game_logic::*;
use crate::components::*;

pub fn turn_start_system(
    mut turn_manager: ResMut<TurnManager>,
    playing_phase: Res<State<PlayingPhase>>,
    mut selectable_query: Query<&mut Selectable>,
) {
    if *playing_phase.get() != PlayingPhase::PlayerTurn {
        return;
    }
    
    if !turn_manager.waiting_for_action {
        turn_manager.waiting_for_action = true;
        
        // Enable selection for current player's cards
        for mut selectable in selectable_query.iter_mut() {
            selectable.enabled = selectable.player_id == turn_manager.current_player;
        }
        
        // info!("Turn {} - Waiting for {:?} to play", 
        //     turn_manager.turn_number, 
        //     turn_manager.current_player
        // );
    }
}

pub fn play_card_system(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    mut action_events: EventReader<PlayerActionEvent>,
    mut capture_events: EventWriter<CaptureEvent>,
    mut next_phase: ResMut<NextState<PlayingPhase>>,
    mut hand_query: Query<(&Player, &mut Hand)>,
    mut table_query: Query<&mut TablePile, With<TableComponent>>,
    mut card_entity_query: Query<&mut CardEntity>,
    card_query: Query<&Card>,
) {
    for event in action_events.read() {
        if event.player_id != turn_manager.current_player {
            // warn!("Player {:?} tried to act out of turn", event.player_id);
            continue;
        }
        
        match &event.action {
            PlayerAction::PlayCard(entity) => {
                // First, get the card info
                let card = if let Ok(card) = card_query.get(*entity) {
                    *card
                } else {
                    continue;
                };
                
                // Remove from player's hand
                let mut card_removed = false;
                for (player, mut hand) in hand_query.iter_mut() {
                    if player.id == event.player_id {
                        if let Ok(_) = hand.remove_card(*entity) {
                            card_removed = true;
                            break;
                        }
                    }
                }
                
                if !card_removed {
                    // warn!("Failed to remove card from player's hand");
                    continue;
                }
                
                // Check for capture
                if let Ok(mut table) = table_query.single_mut() {
                    // Check if we can capture any cards on the table
                    let mut can_capture = false;
                    if !table.cards.is_empty() {
                        for &table_entity in &table.cards {
                            if let Ok(table_card) = card_query.get(table_entity) {
                                if card.can_capture(table_card) {
                                    can_capture = true;
                                    break;
                                }
                            }
                        }
                    }
                    
                    if can_capture {
                        // Determine what cards are captured
                        let mut captured_entities;
                        let mut captured_cards = Vec::new();
                        let was_single_card = table.cards.len() == 1;
                        
                        if card.rank == Rank::Jack {
                            // Jack captures all
                            captured_entities = table.take_all();
                            // Convert entities to cards for the event
                            for entity in &captured_entities {
                                if let Ok(table_card) = card_query.get(*entity) {
                                    captured_cards.push(*table_card);
                                }
                            }
                        } else {
                            // Regular capture - take matching rank
                            let mut entities_to_capture = Vec::new();
                            for &entity in &table.cards {
                                if let Ok(table_card) = card_query.get(entity) {
                                    if table_card.rank == card.rank {
                                        entities_to_capture.push(entity);
                                        captured_cards.push(*table_card);
                                    }
                                }
                            }
                            // Remove captured entities from table
                            table.cards.retain(|&e| !entities_to_capture.contains(&e));
                            captured_entities = entities_to_capture;
                        }
                        
                        // Check for Kseri
                        let is_kseri = was_single_card && captured_cards.len() == 1 && 
                            card.makes_kseri(1);
                        
                        // Check for double Kseri (Jack capturing Jack)
                        let is_double_kseri = is_kseri && 
                            card.rank == Rank::Jack && 
                            captured_cards.iter().any(|c| c.rank == Rank::Jack);
                        
                        // Include the played card in captures
                        captured_cards.push(card);
                        captured_entities.push(*entity);
                        
                        // Send capture event
                        capture_events.write(CaptureEvent {
                            player_id: event.player_id,
                            played_card: card,
                            captured_cards: captured_cards.clone(),
                            captured_entities: captured_entities.clone(),
                            is_kseri,
                            is_double_kseri,
                        });
                        
                        // Update card entity location
                        if let Ok(mut card_entity) = card_entity_query.get_mut(*entity) {
                            card_entity.location = CardLocation::PlayerScore(event.player_id);
                        }
                        
                        // Transition to processing capture
                        next_phase.set(PlayingPhase::ProcessingCapture);
                    } else {
                        // No capture - add to table
                        table.play_card(*entity);
                        
                        // Update card entity
                        if let Ok(mut card_entity) = card_entity_query.get_mut(*entity) {
                            card_entity.location = CardLocation::Table;
                        }
                        commands.entity(*entity)
                            .insert(TablePosition {
                                index: table.cards.len() - 1,
                                position: Vec2::new(0.0, 0.0), // Will be calculated by layout system
                            });
                    }
                    
                    // End turn
                    turn_manager.actions_this_turn += 1;
                    turn_manager.waiting_for_action = false;
                }
            }
        }
    }
}

pub fn turn_end_system(
    mut turn_manager: ResMut<TurnManager>,
    playing_phase: Res<State<PlayingPhase>>,
) {
    if *playing_phase.get() == PlayingPhase::PlayerTurn && 
       turn_manager.actions_this_turn > 0 && 
       !turn_manager.waiting_for_action {
        
        // Switch to next player
        turn_manager.switch_turn();
        // info!("Turn ended. Next player: {:?}", turn_manager.current_player);
    }
}

pub fn process_capture_system(
    mut game_manager: ResMut<GameManager>,
    mut capture_events: EventReader<CaptureEvent>,
    mut score_query: Query<(&Player, &mut Score)>,
) {
    for event in capture_events.read() {
        // info!("Player {:?} captured {} cards{}",
        //     event.player_id,
        //     event.captured_cards.len(),
        //     if event.is_kseri { " (KSERI!)" } else { "" }
        // );
        
        // Update last capture info
        game_manager.last_capture = Some(LastCapture {
            player_id: event.player_id,
            captured_cards: event.captured_cards.clone(),
            was_kseri: event.is_kseri,
        });
        
        // Add to player's score
        for (player, mut score) in score_query.iter_mut() {
            if player.id == event.player_id {
                score.add_collected_cards(event.captured_entities.clone());
                if event.is_double_kseri {
                    score.add_double_kseri();
                } else if event.is_kseri {
                    score.add_kseri();
                }
                break;
            }
        }
    }
}