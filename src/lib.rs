pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;
pub mod game_plugin;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

// WASM entry point
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Log to verify WASM is loading
    web_sys::console::log_1(&"Kseri WASM: Starting initialization...".into());
    
    // This will be called when the wasm module is instantiated
    crate::run();
    
    web_sys::console::log_1(&"Kseri WASM: Initialization complete!".into());
}

// Export the main game logic as a separate function
pub fn run() {
    use bevy::prelude::*;
    use crate::game_plugin::KseriGamePlugin;
    use crate::resources::{GameSettings, NetworkState};
    use crate::systems::{GameState, TurnManager, GameManager};
    use crate::systems::rendering::{
        setup_camera, load_card_textures,
        spawn_card_visuals, update_card_positions, update_card_face, update_card_selection,
        update_card_layering, handle_window_resize, maintain_responsive_layout, CameraScale
    };
    use crate::systems::layout::{LocationCardCounts, update_location_counts};
    use crate::systems::ui::*;
    
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        web_sys::console::log_1(&"Kseri: run() function called".into());
    }
    
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
    
    // Add the game plugin which includes game logic and state management
    app.add_plugins(KseriGamePlugin);
    
    // Add resources
    app.insert_resource(GameSettings {
        player_name: "Arkid".to_string(),
        opponent_name: "Sofia".to_string(),
    });
    app.insert_resource(NetworkState::default());
    app.insert_resource(TurnManager::default());
    
    // Add card rendering plugin
    // Configure card rendering systems
    app
        // Resources
        .init_resource::<LocationCardCounts>()
        .init_resource::<CameraScale>()
        // Startup systems
        .add_systems(Startup, (
            setup_camera,
            load_card_textures,
        ).chain())
        // Update systems
        .add_systems(Update, (
            // Card count tracking must run first
            update_location_counts,
            // Then spawn new cards
            spawn_card_visuals,
            // Then update positions and visuals
            (
                update_card_positions,
                update_card_face,
                update_card_selection,
                update_card_layering,
            ).chain(),
            // Window resize handling
            handle_window_resize,
            maintain_responsive_layout,
        ).chain());
    
    // Add systems
    app.add_systems(Startup, (
        setup,
        setup_ui.after(setup),
    ));
    
    // Add update systems
    app.add_systems(Update, (
        start_game.run_if(resource_exists::<GameManager>),
        update_score_displays,
        update_turn_indicator,
        update_deck_counter,
        update_game_status_messages,
        handle_kseri_event,
        handle_round_end_event,
        handle_game_over_event,
    ));
    
    app.run();
    
    // Local setup function
    fn setup(mut commands: Commands) {
        // Simple text to verify it's working
        commands.spawn((
            Text2d::new("Kseri Game - Press SPACE to start"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 260.0, 0.0),
        ));
    }
    
    fn start_game(
        keys: Res<ButtonInput<KeyCode>>,
        game_state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        if keys.just_pressed(KeyCode::Space) && *game_state.get() == GameState::Menu {
            next_state.set(GameState::GameSetup);
        }
    }
}
