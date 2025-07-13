use bevy::prelude::*;
use tracing::info;
use crate::systems::*;
use crate::components::*;

pub struct KseriGamePlugin;

impl Plugin for KseriGamePlugin {
    fn build(&self, app: &mut App) {
        // Add game states
        app.init_state::<GameState>();
        app.init_state::<PlayingPhase>();
        
        // Add resources
        app.insert_resource(GameManager::new(6)); // 6 rounds per game
        app.init_resource::<TurnManager>();
        app.init_resource::<RoundState>();
        
        // Add events
        app.add_event::<PlayerActionEvent>();
        app.add_event::<CaptureEvent>();
        app.add_event::<RoundEndEvent>();
        app.add_event::<GameStateTransitionEvent>();
        app.add_event::<KseriEvent>();
        app.add_event::<GameOverEvent>();
        
        // Setup systems
        app.add_systems(OnEnter(GameState::GameSetup), setup_game_system);
        
        // Playing state systems
        app.add_systems(
            Update,
            (
                // Dealing phase
                deal_initial_cards.run_if(in_state(PlayingPhase::DealingCards)),
                deal_subsequent_cards.run_if(in_state(PlayingPhase::DealingCards)),
                
                // Player turn phase
                (
                    turn_start_system,
                    play_card_system,
                    check_dealing_needed,
                ).chain().run_if(in_state(PlayingPhase::PlayerTurn)),
                
                // Capture processing
                process_capture_system.run_if(in_state(PlayingPhase::ProcessingCapture)),
                
                // State transitions
                check_game_state_transitions,
                handle_state_transition_events,
                
                // Round end
                handle_round_end.run_if(in_state(PlayingPhase::RoundEnd)),
            )
            .run_if(in_state(GameState::Playing))
        );
        
        // Game over systems
        app.add_systems(
            OnEnter(GameState::GameOver),
            calculate_final_scores
        );
    }
}

/// Calculate final scores including majority bonus
pub fn calculate_final_scores(
    mut score_query: Query<(&Player, &mut Score)>,
    card_query: Query<&Card>,
) {
    // First, calculate base scores for each player
    let mut player_card_counts = Vec::new();
    
    for (player, mut score) in score_query.iter_mut() {
        // Calculate base score from card values and kseris
        score.calculate_score(&card_query);
        player_card_counts.push((player.id, score.card_count()));
    }
    
    // Determine who has majority of cards (>26)
    let mut max_cards = 0;
    let mut majority_player = None;
    let mut tied = false;
    
    for (player_id, count) in &player_card_counts {
        if *count > max_cards {
            max_cards = *count;
            majority_player = Some(*player_id);
            tied = false;
        } else if *count == max_cards {
            tied = true;
        }
    }
    
    // Award majority bonus if not tied and player has >26 cards
    if !tied && max_cards > 26 {
        if let Some(winner_id) = majority_player {
            for (player, mut score) in score_query.iter_mut() {
                if player.id == winner_id {
                    score.total_points += 3;
                    info!("Player {:?} gets majority bonus (+3 points) with {} cards", 
                        winner_id, max_cards);
                    break;
                }
            }
        }
    }
    
    // Log final scores
    for (player, score) in score_query.iter() {
        info!("Final score - {}: {} points ({} cards, {} kseris, {} double kseris)", 
            player.name, score.total_points, score.card_count(), 
            score.kseri_count, score.double_kseri_count);
    }
}