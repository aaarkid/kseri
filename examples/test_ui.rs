use bevy::prelude::*;
use kseri::{systems::*, resources::*, components::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(GameSettings {
            player_name: "Arkid".to_string(),
            opponent_name: "Sofia".to_string(),
        })
        .insert_resource(TurnManager::default())
        .add_event::<KseriEvent>()
        .add_event::<RoundEndEvent>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (setup_camera, setup_ui, setup_test_data))
        .add_systems(Update, (
            update_score_displays,
            update_turn_indicator,
            update_deck_counter,
            update_game_status_messages,
            handle_kseri_event,
            handle_round_end_event,
            handle_game_over_event,
            test_ui_events,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_test_data(mut commands: Commands) {
    // Create test players with scores
    let player1 = commands.spawn((
        Player {
            id: PlayerId::PLAYER_ONE,
            name: "Arkid".to_string(),
            is_local: true,
        },
        Score {
            cards_collected: vec![],
            kseri_count: 2,
            total_points: 25,
        },
    )).id();
    
    let player2 = commands.spawn((
        Player {
            id: PlayerId::PLAYER_TWO,
            name: "Sofia".to_string(),
            is_local: false,
        },
        Score {
            cards_collected: vec![],
            kseri_count: 1,
            total_points: 15,
        },
    )).id();
    
    // Create a test deck
    commands.spawn(Deck::new());
}

fn test_ui_events(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut kseri_events: EventWriter<KseriEvent>,
    mut round_end_events: EventWriter<RoundEndEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut turn_manager: ResMut<TurnManager>,
) {
    // Press K to trigger Kseri event
    if keyboard.just_pressed(KeyCode::KeyK) {
        kseri_events.send(KseriEvent {
            player_id: turn_manager.current_player,
            card: Card::new(Suit::Hearts, Rank::King),
        });
    }
    
    // Press R to trigger round end
    if keyboard.just_pressed(KeyCode::KeyR) {
        round_end_events.send(RoundEndEvent {
            round_number: 1,
            player_scores: [
                (PlayerId::PLAYER_ONE, 25),
                (PlayerId::PLAYER_TWO, 15),
            ],
        });
    }
    
    // Press G to trigger game over
    if keyboard.just_pressed(KeyCode::KeyG) {
        game_over_events.send(GameOverEvent {
            winner: Some(PlayerId::PLAYER_ONE),
            final_scores: [
                (PlayerId::PLAYER_ONE, 52),
                (PlayerId::PLAYER_TWO, 48),
            ],
        });
    }
    
    // Press Space to switch turns
    if keyboard.just_pressed(KeyCode::Space) {
        turn_manager.switch_turn();
    }
}