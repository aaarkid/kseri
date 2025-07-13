use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct NetworkState {
    pub is_connected: bool,
    pub is_host: bool,
}