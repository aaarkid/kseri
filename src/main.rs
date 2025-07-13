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
        player_name: "Player 1".to_string(),
        opponent_name: "Player 2".to_string(),
    });
    app.insert_resource(NetworkState::default());
    
    // Add systems
    app.add_systems(Startup, (setup, setup_deck));
    app.add_systems(Update, handle_card_selection);
    
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    // Simple text to verify it's working
    commands.spawn((
        Text2d::new("Kseri Game - Loading..."),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}
