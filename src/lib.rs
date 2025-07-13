pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;

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
    use crate::resources::{GameSettings, NetworkState};
    use crate::systems::{GameState, setup_deck, deal_cards, handle_card_selection};
    use crate::systems::rendering::{
        setup_camera, load_card_textures,
        spawn_card_visuals, update_card_positions, update_card_face, update_card_selection,
        update_card_layering, handle_window_resize, maintain_responsive_layout, CameraScale
    };
    use crate::systems::layout::{LocationCardCounts, update_location_counts};
    
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
    
    // Add game state
    app.init_state::<GameState>();
    
    // Add resources
    app.insert_resource(GameSettings {
        player_name: "Player 1".to_string(),
        opponent_name: "Player 2".to_string(),
    });
    app.insert_resource(NetworkState::default());
    
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
        setup_deck.after(load_card_textures),
        // Temporary: deal cards immediately for testing
        deal_cards.after(setup_deck),
    ));
    app.add_systems(Update, handle_card_selection);
    
    app.run();
    
    // Local setup function
    fn setup(mut commands: Commands) {
        // Simple text to verify it's working - will be replaced by UI later
        commands.spawn((
            Text2d::new("Kseri Game"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Transform::from_xyz(0.0, 260.0, 0.0),
        ));
    }
}
