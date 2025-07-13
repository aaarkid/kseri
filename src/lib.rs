pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;
