use bevy::prelude::*;
use crate::components::card::{Card, Suit, Rank};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Component, Default)]
pub struct TablePile {
    pub cards: Vec<Entity>,
}

impl TablePile {
    pub fn new() -> Self {
        TablePile {
            cards: Vec::new(),
        }
    }
    
    /// Add a card to the pile (play a card)
    pub fn play_card(&mut self, card: Entity) {
        self.cards.push(card);
    }
    
    /// Add a card to the pile (alias for play_card)
    pub fn add_card(&mut self, card: Entity) {
        self.play_card(card);
    }
    
    /// Take all cards from the pile (for collection)
    pub fn take_all(&mut self) -> Vec<Entity> {
        std::mem::take(&mut self.cards)
    }
    
    /// Peek at the top card without removing it
    pub fn peek_top(&self) -> Option<Entity> {
        self.cards.last().copied()
    }
    
    /// Get the top card entity (alias for peek_top)
    pub fn top_card(&self) -> Option<Entity> {
        self.peek_top()
    }
    
    /// Get the number of cards in the pile
    pub fn count(&self) -> usize {
        self.cards.len()
    }
    
    /// Check if the pile is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    
    /// Clear all cards from the pile
    pub fn clear(&mut self) {
        self.cards.clear();
    }
    
    /// Get an iterator over the card entities
    pub fn iter(&self) -> std::slice::Iter<Entity> {
        self.cards.iter()
    }
    
    /// Get the card at a specific index
    pub fn get(&self, index: usize) -> Option<&Entity> {
        self.cards.get(index)
    }
    
    /// Check if the pile would result in a Kseri capture
    /// (single card on table that gets captured by matching rank)
    pub fn would_be_kseri(&self) -> bool {
        self.cards.len() == 1
    }
}

/// Implement IntoIterator for TablePile to allow easy iteration
impl IntoIterator for TablePile {
    type Item = Entity;
    type IntoIter = std::vec::IntoIter<Entity>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a TablePile {
    type Item = &'a Entity;
    type IntoIter = std::slice::Iter<'a, Entity>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

#[derive(Component)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Create a new deck with all 52 cards
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        
        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(suit, rank));
            }
        }
        
        Deck { cards }
    }
    
    /// Shuffle the deck using Fisher-Yates algorithm
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    /// Draw a card from the top of the deck
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    
    /// Draw multiple cards at once
    pub fn draw_many(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.draw() {
                drawn.push(card);
            } else {
                break;
            }
        }
        drawn
    }
    
    /// Check if deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    
    /// Get remaining card count
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
    
    /// Reset and shuffle the deck
    pub fn reset(&mut self) {
        *self = Deck::new();
        self.shuffle();
    }
}