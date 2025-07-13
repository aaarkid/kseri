use bevy::prelude::*;
use crate::components::card::{Card, CardLocation, CardPosition, Rank, Suit};

/// System to create a standard 52-card deck at startup
pub fn setup_deck(mut commands: Commands) {
    let mut deck_index = 0;
    
    // Create all 52 cards
    for suit in Suit::all() {
        for rank in Rank::all() {
            commands.spawn((
                Card::new(suit, rank),
                CardPosition {
                    location: CardLocation::Deck,
                    index: deck_index,
                },
            ));
            deck_index += 1;
        }
    }
    
    println!("Created deck with {} cards", deck_index);
}

/// System to shuffle the deck
pub fn shuffle_deck(
    mut deck_query: Query<&mut CardPosition, With<Card>>,
) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    
    // Get all deck cards
    let mut deck_entities: Vec<_> = deck_query
        .iter_mut()
        .filter(|pos| matches!(pos.location, CardLocation::Deck))
        .collect();
    
    // Shuffle indices
    let mut indices: Vec<usize> = (0..deck_entities.len()).collect();
    indices.shuffle(&mut thread_rng());
    
    // Apply new indices
    for (pos, &new_index) in deck_entities.iter_mut().zip(indices.iter()) {
        pos.index = new_index;
    }
}

/// Deal cards from deck to player hands
pub fn deal_cards(
    mut deck_query: Query<(Entity, &mut CardPosition), With<Card>>,
) {
    // This is a placeholder - the actual dealing logic should be in game_logic.rs
    // For now, let's just move some cards to test rendering
    
    let mut player_one_count = 0;
    let mut player_two_count = 0;
    let mut table_count = 0;
    
    for (_entity, mut position) in deck_query.iter_mut() {
        if matches!(position.location, CardLocation::Deck) {
            if player_one_count < 4 {
                position.location = CardLocation::PlayerHand(crate::components::card::PlayerId::PLAYER_ONE);
                position.index = player_one_count;
                player_one_count += 1;
            } else if player_two_count < 4 {
                position.location = CardLocation::PlayerHand(crate::components::card::PlayerId::PLAYER_TWO);
                position.index = player_two_count;
                player_two_count += 1;
            } else if table_count < 4 {
                position.location = CardLocation::Table;
                position.index = table_count;
                table_count += 1;
            }
            
            if player_one_count >= 4 && player_two_count >= 4 && table_count >= 4 {
                break;
            }
        }
    }
}