use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::card::{CardLocation, CardPosition, PlayerId};

/// Constants for card layout
pub const CARD_WIDTH: f32 = 64.0;
pub const CARD_HEIGHT: f32 = 96.0;
pub const CARD_SPACING: f32 = 20.0;
pub const HAND_FAN_ANGLE: f32 = std::f32::consts::PI / 6.0; // 30 degrees total
pub const TABLE_CARD_OFFSET_X: f32 = 5.0;
pub const TABLE_CARD_OFFSET_Y: f32 = -2.0;

/// Z-layer constants for proper ordering
pub const Z_LAYER_TABLE: f32 = 0.0;
pub const Z_LAYER_DECK: f32 = 10.0;
pub const Z_LAYER_SCORE: f32 = 50.0;
pub const Z_LAYER_HAND: f32 = 100.0;
pub const Z_LAYER_SELECTED: f32 = 200.0;

/// Screen position constants
pub const DECK_POSITION: Vec3 = Vec3::new(-300.0, 0.0, Z_LAYER_DECK);
pub const TABLE_CENTER: Vec3 = Vec3::new(0.0, 0.0, Z_LAYER_TABLE);
pub const PLAYER_ONE_HAND_Y: f32 = -250.0;
pub const PLAYER_TWO_HAND_Y: f32 = 250.0;
pub const SCORE_PILE_X: f32 = 300.0;
pub const PLAYER_ONE_SCORE_Y: f32 = -200.0;
pub const PLAYER_TWO_SCORE_Y: f32 = 200.0;

/// Resource to track card counts per location
#[derive(Resource, Default)]
pub struct LocationCardCounts {
    pub counts: HashMap<CardLocation, usize>,
}

impl LocationCardCounts {
    /// Update count for a specific location
    pub fn update_count(&mut self, location: CardLocation, count: usize) {
        self.counts.insert(location, count);
    }
    
    /// Get count for a specific location
    pub fn get_count(&self, location: &CardLocation) -> usize {
        self.counts.get(location).copied().unwrap_or(0)
    }
}

/// Component to mark a card as selected
#[derive(Component)]
pub struct SelectedCard;

/// Calculate transform for a card based on its position and location count
pub fn calculate_card_transform(
    position: &CardPosition,
    total_cards: usize,
    is_selected: bool,
) -> Transform {
    let location = &position.location;
    let index = position.index;
    let mut transform = match location {
        CardLocation::Deck => {
            // Stack deck cards with minimal offset
            Transform::from_translation(
                DECK_POSITION + Vec3::new(
                    index as f32 * 0.1,
                    index as f32 * 0.1,
                    index as f32,
                )
            )
        }
        CardLocation::PlayerHand(player_id) => {
            calculate_hand_transform(*player_id, index, total_cards)
        }
        CardLocation::Table => {
            // Cascade table cards for visibility
            Transform::from_translation(
                TABLE_CENTER + Vec3::new(
                    index as f32 * TABLE_CARD_OFFSET_X,
                    index as f32 * TABLE_CARD_OFFSET_Y,
                    index as f32,
                )
            )
        }
        CardLocation::PlayerScore(player_id) => {
            // Stack score cards neatly
            let y = if *player_id == PlayerId::PLAYER_ONE {
                PLAYER_ONE_SCORE_Y
            } else {
                PLAYER_TWO_SCORE_Y
            };
            Transform::from_translation(Vec3::new(
                SCORE_PILE_X,
                y,
                Z_LAYER_SCORE + index as f32 * 0.1,
            ))
        }
    };
    
    // Apply selected card z-layer if needed
    if is_selected {
        transform.translation.z = Z_LAYER_SELECTED;
    }
    
    transform
}

/// Calculate transform for cards in hand with fan effect
fn calculate_hand_transform(
    player_id: PlayerId,
    index: usize,
    total_cards: usize,
) -> Transform {
    let y = if player_id == PlayerId::PLAYER_ONE {
        PLAYER_ONE_HAND_Y
    } else {
        PLAYER_TWO_HAND_Y
    };
    
    if total_cards == 0 {
        return Transform::from_translation(Vec3::new(0.0, y, Z_LAYER_HAND));
    }
    
    // Calculate fan layout
    let cards_f32 = total_cards as f32;
    let spacing = if total_cards > 1 {
        (CARD_WIDTH + CARD_SPACING).min(400.0 / cards_f32)
    } else {
        0.0
    };
    
    // Center the hand
    let total_width = spacing * (cards_f32 - 1.0);
    let start_x = -total_width / 2.0;
    let x = start_x + index as f32 * spacing;
    
    // Calculate rotation for fan effect
    let angle = if total_cards > 1 {
        let angle_step = HAND_FAN_ANGLE / (cards_f32 - 1.0);
        -HAND_FAN_ANGLE / 2.0 + angle_step * index as f32
    } else {
        0.0
    };
    
    // Apply a slight arc to the hand
    let arc_height = if player_id == PlayerId::PLAYER_ONE { -10.0 } else { 10.0 };
    let arc_offset = arc_height * (1.0 - ((index as f32 - (cards_f32 - 1.0) / 2.0) / (cards_f32 / 2.0)).powi(2));
    
    Transform::from_translation(Vec3::new(x, y + arc_offset, Z_LAYER_HAND + index as f32))
        .with_rotation(Quat::from_rotation_z(angle))
}

/// System to update card counts when positions change
pub fn update_location_counts(
    mut counts: ResMut<LocationCardCounts>,
    query: Query<&CardPosition>,
) {
    // Clear and recount all locations
    counts.counts.clear();
    
    // Count all cards per location
    let mut location_counts: HashMap<CardLocation, usize> = HashMap::new();
    
    for position in query.iter() {
        *location_counts.entry(position.location).or_insert(0) += 1;
    }
    
    // Update resource
    for (location, count) in location_counts {
        counts.update_count(location, count);
    }
}

/// System to update selected card z-ordering
pub fn update_selected_card_z(
    mut query: Query<&mut Transform, (With<SelectedCard>, Changed<SelectedCard>)>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.z = Z_LAYER_SELECTED;
    }
}

/// System to maintain proper z-ordering for table cards
pub fn maintain_table_z_order(
    mut query: Query<(&CardPosition, &mut Transform), Changed<CardPosition>>,
) {
    for (position, mut transform) in query.iter_mut() {
        if matches!(position.location, CardLocation::Table) {
            // Ensure table cards maintain cascading z-order
            transform.translation.z = position.index as f32;
        }
    }
}