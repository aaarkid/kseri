use bevy::prelude::*;
use crate::components::{Player, Score, PlayerId};
use crate::resources::GameSettings;

#[derive(Component)]
pub struct UIRoot;

#[derive(Component)]
pub struct PlayerNameDisplay {
    pub player_id: PlayerId,
}

#[derive(Component)]
pub struct ScoreDisplay {
    pub player_id: PlayerId,
}

#[derive(Component)]
pub struct TurnIndicator;

#[derive(Component)]
pub struct DeckCounter;

#[derive(Component)]
pub struct GameStatusMessage {
    pub timer: Timer,
    pub fade_out: bool,
}

#[derive(Component)]
pub struct KseriBanner;

pub fn setup_ui(
    mut commands: Commands,
    settings: Res<GameSettings>,
) {
    // Create UI root entity
    let ui_root = commands.spawn((
        UIRoot,
        Transform::from_xyz(0.0, 0.0, 200.0), // UI layer at z=200
        Visibility::default(),
    )).id();
    
    // Player 1 (Arkid) - bottom of screen
    let player1_name = commands.spawn((
        PlayerNameDisplay { player_id: PlayerId::PLAYER_ONE },
        Text2d::new(&settings.player_name),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.2, 0.5)), // Deep purple
        Transform::from_xyz(-250.0, -250.0, 201.0),
    )).id();
    commands.entity(ui_root).add_child(player1_name);
    
    let player1_score = commands.spawn((
        ScoreDisplay { player_id: PlayerId::PLAYER_ONE },
        Text2d::new("Score: 0"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.2, 0.5)), // Deep purple
        Transform::from_xyz(-250.0, -280.0, 201.0),
    )).id();
    commands.entity(ui_root).add_child(player1_score);
    
    // Player 2 (Sofia) - top of screen
    let player2_name = commands.spawn((
        PlayerNameDisplay { player_id: PlayerId::PLAYER_TWO },
        Text2d::new(&settings.opponent_name),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.2, 0.5)), // Deep purple
        Transform::from_xyz(250.0, 250.0, 201.0),
    )).id();
    commands.entity(ui_root).add_child(player2_name);
    
    let player2_score = commands.spawn((
        ScoreDisplay { player_id: PlayerId::PLAYER_TWO },
        Text2d::new("Score: 0"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.2, 0.5)), // Deep purple
        Transform::from_xyz(250.0, 220.0, 201.0),
    )).id();
    commands.entity(ui_root).add_child(player2_score);
    
    // Turn indicator - starts at player 1
    let turn_indicator = commands.spawn((
        TurnIndicator,
        Text2d::new("▶"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.6, 0.2)), // Goldenrod
        Transform::from_xyz(-320.0, -250.0, 202.0),
    )).id();
    commands.entity(ui_root).add_child(turn_indicator);
    
    // Deck counter - centered on left side
    let deck_counter = commands.spawn((
        DeckCounter,
        Text2d::new("Deck: 40"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.2, 0.5)), // Deep purple
        Transform::from_xyz(-350.0, 0.0, 201.0),
    )).id();
    commands.entity(ui_root).add_child(deck_counter);
}

pub fn update_score_displays(
    score_query: Query<(&Score, &Player)>,
    mut text_query: Query<(&mut Text2d, &ScoreDisplay)>,
) {
    for (mut text, score_display) in text_query.iter_mut() {
        for (score, player) in score_query.iter() {
            if player.id == score_display.player_id {
                text.0 = format!("Score: {}", score.total_points);
            }
        }
    }
}

pub fn update_turn_indicator(
    turn_manager: Res<crate::systems::TurnManager>,
    mut indicator_query: Query<&mut Transform, With<TurnIndicator>>,
) {
    if let Ok(mut transform) = indicator_query.single_mut() {
        match turn_manager.current_player {
            PlayerId::PLAYER_ONE => {
                transform.translation.x = -320.0;
                transform.translation.y = -250.0;
            }
            PlayerId::PLAYER_TWO => {
                transform.translation.x = 180.0;
                transform.translation.y = 250.0;
            }
            _ => {} // Handle other player IDs if needed
        }
    }
}

pub fn update_deck_counter(
    deck_query: Query<&crate::components::Deck>,
    mut text_query: Query<&mut Text2d, With<DeckCounter>>,
) {
    if let Ok(deck) = deck_query.single() {
        if let Ok(mut text) = text_query.single_mut() {
            text.0 = format!("Deck: {}", deck.remaining());
        }
    }
}

pub fn spawn_game_status_message(
    commands: &mut Commands,
    message: &str,
    duration: f32,
) -> Entity {
    commands.spawn((
        GameStatusMessage {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            fade_out: false,
        },
        Text2d::new(message),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Transform::from_xyz(0.0, 0.0, 250.0), // Above everything
    )).id()
}

pub fn update_game_status_messages(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GameStatusMessage, &mut TextColor)>,
) {
    for (entity, mut status, mut color) in query.iter_mut() {
        status.timer.tick(time.delta());
        
        if status.timer.finished() && !status.fade_out {
            status.fade_out = true;
            status.timer = Timer::from_seconds(0.5, TimerMode::Once);
        } else if status.fade_out {
            let alpha = 1.0 - status.timer.fraction();
            color.0 = color.0.with_alpha(alpha);
            
            if status.timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn handle_kseri_event(
    mut commands: Commands,
    mut kseri_events: EventReader<crate::systems::KseriEvent>,
) {
    for _event in kseri_events.read() {
        spawn_game_status_message(&mut commands, "KSERI!", 2.0);
    }
}

pub fn handle_round_end_event(
    mut commands: Commands,
    mut round_end_events: EventReader<crate::systems::RoundEndEvent>,
) {
    for _event in round_end_events.read() {
        spawn_game_status_message(&mut commands, "Round Complete", 3.0);
    }
}

pub fn handle_game_over_event(
    mut commands: Commands,
    mut game_over_events: EventReader<crate::systems::GameOverEvent>,
    score_query: Query<(&Score, &Player)>,
) {
    for _event in game_over_events.read() {
        let mut player1_score = 0;
        let mut player2_score = 0;
        let mut player1_name = "Player 1";
        let mut player2_name = "Player 2";
        
        for (score, player) in score_query.iter() {
            match player.id {
                PlayerId::PLAYER_ONE => {
                    player1_score = score.total_points;
                    player1_name = &player.name;
                }
                PlayerId::PLAYER_TWO => {
                    player2_score = score.total_points;
                    player2_name = &player.name;
                }
                _ => {} // Handle other player IDs if needed
            }
        }
        
        let winner = if player1_score > player2_score {
            player1_name
        } else if player2_score > player1_score {
            player2_name
        } else {
            "It's a tie"
        };
        
        let message = format!("Game Over!\n{} wins!", winner);
        spawn_game_status_message(&mut commands, &message, 5.0);
    }
}