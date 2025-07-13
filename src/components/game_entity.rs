use bevy::prelude::*;
use crate::components::{Card, PlayerId, CardLocation};

#[derive(Component)]
pub struct CardEntity {
    pub card: Card,
    pub location: CardLocation,
}

#[derive(Component, Default)]
pub struct TablePosition {
    pub index: usize,
    pub position: Vec2,
}

#[derive(Component)]
pub struct AnimationState {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub progress: f32,
    pub duration: f32,
}

#[derive(Component)]
pub struct Selectable {
    pub player_id: PlayerId,
    pub enabled: bool,
}

#[derive(Component)]
pub struct DeckComponent;

#[derive(Component)]
pub struct TableComponent;

#[derive(Component)]
pub struct PlayerHandComponent {
    pub player_id: PlayerId,
}

#[derive(Component)]
pub struct ScorePileComponent {
    pub player_id: PlayerId,
}

impl AnimationState {
    pub fn new(start_pos: Vec3, end_pos: Vec3, duration: f32) -> Self {
        Self {
            start_pos,
            end_pos,
            progress: 0.0,
            duration,
        }
    }

    pub fn update(&mut self, delta: f32) -> bool {
        self.progress += delta / self.duration;
        self.progress >= 1.0
    }

    pub fn current_position(&self) -> Vec3 {
        self.start_pos.lerp(self.end_pos, self.progress.clamp(0.0, 1.0))
    }
}