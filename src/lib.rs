pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;
pub mod game_plugin;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;
