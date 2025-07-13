use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod components;
mod systems;
mod resources;
mod game_plugin;

use game_plugin::KseriGamePlugin;
use resources::*;
use systems::GameState;

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
    
    // Add the game plugin
    app.add_plugins(KseriGamePlugin);
    
    // Add resources
    app.insert_resource(GameSettings {
        player_name: "Arkid".to_string(),
        opponent_name: "Sofia".to_string(),
    });
    app.insert_resource(NetworkState::default());
    
    // Add basic setup systems
    app.add_systems(Startup, setup);
    
    // Start the game
    app.add_systems(Update, start_game.run_if(resource_exists::<GameManager>));
    
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    // Simple text to verify it's working
    commands.spawn((
        Text2d::new("Kseri Game - Press SPACE to start"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn start_game(
    keys: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) && *game_state.get() == GameState::MainMenu {
        next_state.set(GameState::GameSetup);
    }
}
