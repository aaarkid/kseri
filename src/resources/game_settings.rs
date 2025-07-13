use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub player_name: String,
    pub opponent_name: String,
}