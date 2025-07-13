use bevy::prelude::*;
use crate::components::card::{PlayerId, Card};
use std::fmt;

#[derive(Component)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub is_local: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HandError {
    CapacityExceeded,
    CardNotFound,
    InvalidIndex,
}

impl fmt::Display for HandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandError::CapacityExceeded => write!(f, "Hand capacity exceeded"),
            HandError::CardNotFound => write!(f, "Card not found in hand"),
            HandError::InvalidIndex => write!(f, "Invalid card index"),
        }
    }
}

impl std::error::Error for HandError {}

#[derive(Component, Default)]
pub struct Hand {
    pub cards: Vec<Entity>,
    pub max_capacity: usize,
}

impl Hand {
    /// Create a new hand with specified maximum capacity
    pub fn new(max_capacity: usize) -> Self {
        Hand {
            cards: Vec::with_capacity(max_capacity),
            max_capacity,
        }
    }
    
    /// Add a card to the hand
    pub fn add_card(&mut self, card: Entity) -> Result<(), HandError> {
        if self.cards.len() >= self.max_capacity {
            return Err(HandError::CapacityExceeded);
        }
        self.cards.push(card);
        Ok(())
    }
    
    /// Remove a card from the hand by entity
    pub fn remove_card(&mut self, card: Entity) -> Result<(), HandError> {
        if let Some(pos) = self.cards.iter().position(|&c| c == card) {
            self.cards.remove(pos);
            Ok(())
        } else {
            Err(HandError::CardNotFound)
        }
    }
    
    /// Remove a card by index
    pub fn remove_card_at(&mut self, index: usize) -> Result<Entity, HandError> {
        if index >= self.cards.len() {
            return Err(HandError::InvalidIndex);
        }
        Ok(self.cards.remove(index))
    }
    
    /// Get a card at specific index without removing it
    pub fn get_card(&self, index: usize) -> Option<&Entity> {
        self.cards.get(index)
    }
    
    /// Check if hand contains a specific card
    pub fn contains(&self, card: Entity) -> bool {
        self.cards.contains(&card)
    }
    
    /// Get the number of cards in hand
    pub fn count(&self) -> usize {
        self.cards.len()
    }
    
    /// Check if hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    
    /// Check if hand is full
    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_capacity
    }
    
    /// Clear all cards from hand
    pub fn clear(&mut self) {
        self.cards.clear();
    }
    
    /// Get remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        self.max_capacity.saturating_sub(self.cards.len())
    }
    
    /// Sort cards by a custom comparison function
    /// This is useful for organizing cards in the UI
    pub fn sort_by<F>(&mut self, compare: F) 
    where
        F: FnMut(&Entity, &Entity) -> std::cmp::Ordering,
    {
        self.cards.sort_by(compare);
    }
    
    /// Get an iterator over the cards
    pub fn iter(&self) -> std::slice::Iter<Entity> {
        self.cards.iter()
    }
    
    /// Get a mutable iterator over the cards
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Entity> {
        self.cards.iter_mut()
    }
}

/// Helper function to sort cards in a hand by suit and rank
/// Requires access to Card components through a query
pub fn sort_hand_by_suit_and_rank(
    hand: &mut Hand,
    card_query: &Query<&Card>,
) {
    hand.sort_by(|&a, &b| {
        let card_a = card_query.get(a).ok();
        let card_b = card_query.get(b).ok();
        
        match (card_a, card_b) {
            (Some(a), Some(b)) => {
                // First compare by suit, then by rank
                match a.suit.cmp(&b.suit) {
                    std::cmp::Ordering::Equal => a.rank.cmp(&b.rank),
                    other => other,
                }
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
}

#[derive(Component, Default)]
pub struct Score {
    pub cards_collected: Vec<Entity>,
    pub kseri_count: u32,
    pub double_kseri_count: u32,
    pub total_points: u32,
}

impl Score {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add collected cards to the score pile
    pub fn add_collected_cards(&mut self, cards: Vec<Entity>) {
        self.cards_collected.extend(cards);
    }
    
    /// Increment kseri count
    pub fn add_kseri(&mut self) {
        self.kseri_count += 1;
    }
    
    /// Increment double kseri count
    pub fn add_double_kseri(&mut self) {
        self.double_kseri_count += 1;
    }
    
    /// Get the number of collected cards
    pub fn card_count(&self) -> usize {
        self.cards_collected.len()
    }
    
    /// Calculate score based on collected cards
    /// This requires access to Card components
    pub fn calculate_score(&mut self, card_query: &Query<&Card>) -> u32 {
        let mut points = 0;
        
        // Count special cards
        for &card_entity in &self.cards_collected {
            if let Ok(card) = card_query.get(card_entity) {
                points += card.kseri_value();
            }
        }
        
        // Add Kseri bonuses (10 points each for regular, 20 for double)
        points += self.kseri_count * 10;
        points += self.double_kseri_count * 20;
        
        self.total_points = points;
        points
    }
}