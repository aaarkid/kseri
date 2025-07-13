use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod components;
mod systems;
mod resources;

use systems::*;
use resources::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Kseri".to_string(),
            resolution: (800., 600.).into(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy".to_string()),
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));
    
    // Add game state
    app.init_state::<GameState>();
    
    // Add resources
    app.insert_resource(GameSettings {
        player_name: "Arkid".to_string(),
        opponent_name: "Sofia".to_string(),
    });
    app.insert_resource(NetworkState::default());
    app.insert_resource(TurnManager::default());
    
    // Add events
    app.add_event::<PlayerActionEvent>();
    app.add_event::<CaptureEvent>();
    app.add_event::<RoundEndEvent>();
    app.add_event::<GameStateTransitionEvent>();
    app.add_event::<KseriEvent>();
    app.add_event::<GameOverEvent>();
    
    // Add systems
    app.add_systems(Startup, (setup, setup_deck, setup_ui));
    app.add_systems(Update, (
        handle_card_selection,
        update_score_displays,
        update_turn_indicator,
        update_deck_counter,
        update_game_status_messages,
        handle_kseri_event,
        handle_round_end_event,
        handle_game_over_event,
    ));
    
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
