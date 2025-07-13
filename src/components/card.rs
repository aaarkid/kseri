use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }
    
    pub fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }
    
    pub fn value(&self) -> u8 {
        *self as u8
    }
    
    pub fn symbol(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
    
    /// Special scoring value for Kseri game
    pub fn kseri_points(&self) -> u32 {
        match self {
            Rank::Ace => 1,           // Each Ace is worth 1 point
            Rank::Two => 2,           // Two of Clubs is worth 2 points (special card)
            Rank::Ten => 3,           // Ten of Diamonds is worth 3 points (special card)
            Rank::Jack => 1,          // Jack of Diamonds is worth 1 point
            _ => 0,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
    
    /// Check if this card can capture the given card in Kseri
    pub fn can_capture(&self, other: &Card) -> bool {
        // In Kseri, cards capture by matching rank
        self.rank == other.rank || self.rank == Rank::Jack
    }
    
    /// Get the special scoring value for this card in Kseri
    pub fn kseri_value(&self) -> u32 {
        match (self.suit, self.rank) {
            (Suit::Clubs, Rank::Two) => 2,      // Two of Clubs = 2 points
            (Suit::Diamonds, Rank::Ten) => 3,   // Ten of Diamonds = 3 points
            (Suit::Diamonds, Rank::Jack) => 1,  // Jack of Diamonds = 1 point
            (_, Rank::Ace) => 1,                // Each Ace = 1 point
            _ => 0,
        }
    }
    
    /// Check if capturing with this card results in a Kseri
    pub fn makes_kseri(&self, table_count: usize) -> bool {
        // Kseri happens when you capture a single card with a card of the same rank
        // (except when using a Jack)
        table_count == 1 && self.rank != Rank::Jack
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Component)]
pub struct CardPosition {
    pub location: CardLocation,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardLocation {
    Deck,
    PlayerHand(PlayerId),
    Table,
    PlayerScore(PlayerId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub const PLAYER_ONE: PlayerId = PlayerId(0);
    pub const PLAYER_TWO: PlayerId = PlayerId(1);
}