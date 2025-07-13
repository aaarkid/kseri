use bevy::prelude::*;

pub fn handle_card_selection(
    buttons: Res<ButtonInput<MouseButton>>,
    _windows: Query<&Window>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Placeholder for card selection logic
    }
}