use bevy::prelude::*;
use bevy::window::WindowResized;
use std::collections::HashMap;
use crate::components::card::{Card, CardPosition, Rank, Suit};
use crate::systems::layout::{calculate_card_transform, LocationCardCounts};

/// Resource containing all card texture handles
#[derive(Resource)]
pub struct CardTextures {
    /// Map from Card to its texture handle
    pub cards: HashMap<Card, Handle<Image>>,
    /// Card back texture
    pub card_back: Handle<Image>,
}

impl CardTextures {
    /// Get the texture handle for a specific card
    pub fn get_card_texture(&self, card: &Card) -> Handle<Image> {
        self.cards.get(card).cloned().unwrap_or_else(|| self.card_back.clone())
    }
}

/// Component to track card visual state
#[derive(Component)]
pub struct CardVisual {
    pub face_up: bool,
    pub selected: bool,
}

impl Default for CardVisual {
    fn default() -> Self {
        CardVisual {
            face_up: true,
            selected: false,
        }
    }
}

/// System to load all card textures at startup
pub fn load_card_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut cards_map = HashMap::new();
    
    // Load textures for all card combinations
    for suit in Suit::all() {
        for rank in Rank::all() {
            let card = Card::new(suit, rank);
            let filename = get_card_filename(&card);
            let texture_handle: Handle<Image> = asset_server.load(filename);
            cards_map.insert(card, texture_handle);
        }
    }
    
    // Load card back texture
    let card_back = asset_server.load("cards/individual/card_back.png");
    
    // Insert as a resource
    commands.insert_resource(CardTextures {
        cards: cards_map,
        card_back,
    });
}

/// Get the filename for a card's texture
fn get_card_filename(card: &Card) -> String {
    let suit_name = match card.suit {
        Suit::Hearts => "hearts",
        Suit::Diamonds => "diamonds",
        Suit::Clubs => "clubs",
        Suit::Spades => "spades",
    };
    
    let rank_name = match card.rank {
        Rank::Ace => "ace",
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "jack",
        Rank::Queen => "queen",
        Rank::King => "king",
    };
    
    format!("cards/individual/{}_{}.png", suit_name, rank_name)
}

/// Bundle for spawning card entities
#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub position: CardPosition,
    pub visual: CardVisual,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl CardBundle {
    /// Create a new card bundle with appropriate texture
    pub fn new(
        card: Card,
        position: CardPosition,
        texture: Handle<Image>,
        face_up: bool,
    ) -> Self {
        let visual = CardVisual {
            face_up,
            selected: false,
        };
        
        // Start with default transform, will be updated by layout system
        let transform = Transform::default();
        
        CardBundle {
            card,
            position,
            visual,
            sprite: Sprite {
                image: texture,
                custom_size: Some(Vec2::new(
                    crate::systems::layout::CARD_WIDTH,
                    crate::systems::layout::CARD_HEIGHT,
                )),
                ..Default::default()
            },
            transform,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

/// System to spawn visual representations for cards
pub fn spawn_card_visuals(
    mut commands: Commands,
    card_textures: Res<CardTextures>,
    card_counts: Res<LocationCardCounts>,
    query: Query<(Entity, &Card, &CardPosition, Option<&CardVisual>), (Added<Card>, Without<Sprite>)>,
) {
    for (entity, card, position, maybe_visual) in query.iter() {
        
        // Determine if card should be face up
        let face_up = maybe_visual.map_or(true, |v| v.face_up);
        
        // Get the appropriate texture
        let texture = if face_up {
            card_textures.get_card_texture(card)
        } else {
            card_textures.card_back.clone()
        };
        
        // Calculate initial position
        let total_cards = card_counts.get_count(&position.location);
        let transform = calculate_card_transform(position, total_cards, false);
        
        // Add sprite and visual components to the entity
        commands.entity(entity).insert((
            CardVisual {
                face_up,
                selected: false,
            },
            Sprite {
                image: texture,
                custom_size: Some(Vec2::new(
                    crate::systems::layout::CARD_WIDTH,
                    crate::systems::layout::CARD_HEIGHT,
                )),
                ..Default::default()
            },
            transform,
        ));
    }
}

/// System to update card visuals when their position changes
pub fn update_card_positions(
    card_counts: Res<LocationCardCounts>,
    mut query: Query<(&CardPosition, &mut Transform, &CardVisual), Changed<CardPosition>>,
) {
    for (position, mut transform, visual) in query.iter_mut() {
        let total_cards = card_counts.get_count(&position.location);
        *transform = calculate_card_transform(position, total_cards, visual.selected);
    }
}

/// System to update card face up/down state
pub fn update_card_face(
    card_textures: Res<CardTextures>,
    mut query: Query<(&Card, &CardVisual, &mut Sprite), Changed<CardVisual>>,
) {
    for (card, visual, mut sprite) in query.iter_mut() {
        sprite.image = if visual.face_up {
            card_textures.get_card_texture(card)
        } else {
            card_textures.card_back.clone()
        };
    }
}

/// System to handle card selection visual feedback
pub fn update_card_selection(
    mut query: Query<(&mut Transform, &CardVisual, &CardPosition), Changed<CardVisual>>,
    card_counts: Res<LocationCardCounts>,
) {
    for (mut transform, visual, position) in query.iter_mut() {
        let total_cards = card_counts.get_count(&position.location);
        *transform = calculate_card_transform(position, total_cards, visual.selected);
    }
}

/// Resource to track camera scaling for responsive design
#[derive(Resource)]
pub struct CameraScale {
    pub scale: f32,
    pub base_width: f32,
    pub base_height: f32,
}

impl Default for CameraScale {
    fn default() -> Self {
        CameraScale {
            scale: 1.0,
            base_width: 800.0,
            base_height: 600.0,
        }
    }
}

/// Camera setup system with proper configuration
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Name::new("Main Camera"),
    ));
}

/// System to handle window resize events
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut camera_scale: ResMut<CameraScale>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    for event in resize_events.read() {
        // Calculate scale to maintain aspect ratio
        let width_scale = event.width / camera_scale.base_width;
        let height_scale = event.height / camera_scale.base_height;
        
        // Use the smaller scale to ensure everything fits
        let scale = width_scale.min(height_scale);
        camera_scale.scale = scale;
        
        // Update camera scale (inverse because we're scaling the world, not the camera)
        for mut transform in camera_query.iter_mut() {
            transform.scale = Vec3::splat(1.0 / scale);
        }
    }
}

/// System to maintain responsive layout for UI elements
pub fn maintain_responsive_layout(
    camera_scale: Res<CameraScale>,
    windows: Query<&Window>,
    mut text_query: Query<&mut Transform, With<Text2d>>,
) {
    if let Ok(window) = windows.single() {
        // Update text position to stay at top of screen
        for mut transform in text_query.iter_mut() {
            let scaled_top = (window.height() / 2.0) / camera_scale.scale;
            transform.translation.y = scaled_top - 40.0; // 40 pixels from top
        }
    }
}


/// System to update card layering based on position
pub fn update_card_layering(
    mut query: Query<(&CardPosition, &mut Transform), Changed<CardPosition>>,
) {
    for (position, mut transform) in query.iter_mut() {
        use crate::components::card::CardLocation;
        use crate::systems::layout::{Z_LAYER_DECK, Z_LAYER_HAND, Z_LAYER_SCORE, Z_LAYER_TABLE};
        
        // Set base z-layer based on location
        let base_z = match position.location {
            CardLocation::Deck => Z_LAYER_DECK,
            CardLocation::PlayerHand(_) => Z_LAYER_HAND,
            CardLocation::Table => Z_LAYER_TABLE,
            CardLocation::PlayerScore(_) => Z_LAYER_SCORE,
        };
        
        // Add index offset for proper stacking within location
        transform.translation.z = base_z + position.index as f32 * 0.1;
    }
}