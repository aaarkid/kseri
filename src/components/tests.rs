#[cfg(test)]
mod card_tests {
    use super::super::card::*;
    
    #[test]
    fn test_card_creation() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.rank, Rank::Ace);
    }
    
    #[test]
    fn test_card_display() {
        let card = Card::new(Suit::Spades, Rank::King);
        assert_eq!(format!("{}", card), "K♠");
        
        let card2 = Card::new(Suit::Hearts, Rank::Ten);
        assert_eq!(format!("{}", card2), "10♥");
    }
    
    #[test]
    fn test_can_capture() {
        let jack = Card::new(Suit::Hearts, Rank::Jack);
        let five = Card::new(Suit::Diamonds, Rank::Five);
        let another_five = Card::new(Suit::Clubs, Rank::Five);
        let king = Card::new(Suit::Spades, Rank::King);
        
        // Jack can capture anything
        assert!(jack.can_capture(&five));
        assert!(jack.can_capture(&king));
        
        // Same rank can capture
        assert!(five.can_capture(&another_five));
        
        // Different ranks cannot capture (except Jack)
        assert!(!five.can_capture(&king));
    }
    
    #[test]
    fn test_kseri_values() {
        // Special cards
        assert_eq!(Card::new(Suit::Clubs, Rank::Two).kseri_value(), 2);
        assert_eq!(Card::new(Suit::Diamonds, Rank::Ten).kseri_value(), 3);
        assert_eq!(Card::new(Suit::Diamonds, Rank::Jack).kseri_value(), 1);
        
        // All aces worth 1
        assert_eq!(Card::new(Suit::Hearts, Rank::Ace).kseri_value(), 1);
        assert_eq!(Card::new(Suit::Spades, Rank::Ace).kseri_value(), 1);
        
        // Regular cards worth 0
        assert_eq!(Card::new(Suit::Hearts, Rank::Five).kseri_value(), 0);
    }
    
    #[test]
    fn test_makes_kseri() {
        let five = Card::new(Suit::Hearts, Rank::Five);
        let jack = Card::new(Suit::Spades, Rank::Jack);
        
        // Capturing single card with matching rank makes kseri
        assert!(five.makes_kseri(1));
        
        // Jack never makes kseri
        assert!(!jack.makes_kseri(1));
        
        // Multiple cards on table = no kseri
        assert!(!five.makes_kseri(3));
    }
}

#[cfg(test)]
mod deck_tests {
    use super::super::table::*;
    use super::super::card::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
        assert!(!deck.is_empty());
    }
    
    #[test]
    fn test_deck_contains_all_cards() {
        let deck = Deck::new();
        let mut card_set = HashSet::new();
        
        // Add all cards to set
        for card in &deck.cards {
            card_set.insert((card.suit, card.rank));
        }
        
        // Should have exactly 52 unique cards
        assert_eq!(card_set.len(), 52);
        
        // Check all combinations exist
        for suit in Suit::all() {
            for rank in Rank::all() {
                assert!(card_set.contains(&(suit, rank)));
            }
        }
    }
    
    #[test]
    fn test_deck_draw() {
        let mut deck = Deck::new();
        
        // Draw one card
        let card = deck.draw();
        assert!(card.is_some());
        assert_eq!(deck.remaining(), 51);
        
        // Draw many cards
        let cards = deck.draw_many(10);
        assert_eq!(cards.len(), 10);
        assert_eq!(deck.remaining(), 41);
        
        // Draw remaining cards
        let remaining = deck.draw_many(100);
        assert_eq!(remaining.len(), 41);
        assert!(deck.is_empty());
        
        // Drawing from empty deck
        assert!(deck.draw().is_none());
    }
    
    #[test]
    fn test_deck_shuffle_changes_order() {
        let deck1 = Deck::new();
        let mut deck2 = Deck::new();
        
        // Save original order
        let original: Vec<Card> = deck1.cards.clone();
        
        // Shuffle one deck
        deck2.shuffle();
        
        // Very unlikely they're in the same order after shuffle
        // (1 in 52! chance)
        assert_ne!(original, deck2.cards);
        
        // But should still have same cards
        let mut sorted1 = original;
        let mut sorted2 = deck2.cards.clone();
        sorted1.sort_by_key(|c| (c.suit as u8, c.rank as u8));
        sorted2.sort_by_key(|c| (c.suit as u8, c.rank as u8));
        assert_eq!(sorted1, sorted2);
    }
}

#[cfg(test)]
mod hand_tests {
    use super::super::player::*;
    use super::super::card::*;
    use bevy::prelude::*;
    
    fn create_test_entity() -> Entity {
        Entity::from_raw(1)
    }
    
    #[test]
    fn test_hand_creation() {
        let hand = Hand::new(4);
        assert_eq!(hand.max_capacity, 4);
        assert_eq!(hand.count(), 0);
        assert!(hand.is_empty());
        assert!(!hand.is_full());
    }
    
    #[test]
    fn test_add_card() {
        let mut hand = Hand::new(3);
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        let card3 = Entity::from_raw(3);
        let card4 = Entity::from_raw(4);
        
        // Add cards up to capacity
        assert!(hand.add_card(card1).is_ok());
        assert_eq!(hand.count(), 1);
        
        assert!(hand.add_card(card2).is_ok());
        assert_eq!(hand.count(), 2);
        
        assert!(hand.add_card(card3).is_ok());
        assert_eq!(hand.count(), 3);
        assert!(hand.is_full());
        
        // Try to exceed capacity
        assert!(matches!(hand.add_card(card4), Err(HandError::CapacityExceeded)));
        assert_eq!(hand.count(), 3);
    }
    
    #[test]
    fn test_remove_card() {
        let mut hand = Hand::new(4);
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        let card3 = Entity::from_raw(3);
        
        // Add cards
        hand.add_card(card1).unwrap();
        hand.add_card(card2).unwrap();
        hand.add_card(card3).unwrap();
        
        // Remove existing card
        assert!(hand.remove_card(card2).is_ok());
        assert_eq!(hand.count(), 2);
        assert!(!hand.contains(card2));
        
        // Try to remove non-existent card
        assert!(matches!(hand.remove_card(Entity::from_raw(99)), Err(HandError::CardNotFound)));
    }
    
    #[test]
    fn test_remove_card_at() {
        let mut hand = Hand::new(4);
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        let card3 = Entity::from_raw(3);
        
        hand.add_card(card1).unwrap();
        hand.add_card(card2).unwrap();
        hand.add_card(card3).unwrap();
        
        // Remove by valid index
        let removed = hand.remove_card_at(1).unwrap();
        assert_eq!(removed, card2);
        assert_eq!(hand.count(), 2);
        
        // Try invalid index
        assert!(matches!(hand.remove_card_at(10), Err(HandError::InvalidIndex)));
    }
    
    #[test]
    fn test_hand_queries() {
        let mut hand = Hand::new(4);
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        
        hand.add_card(card1).unwrap();
        hand.add_card(card2).unwrap();
        
        // Test get_card
        assert_eq!(hand.get_card(0), Some(&card1));
        assert_eq!(hand.get_card(1), Some(&card2));
        assert_eq!(hand.get_card(2), None);
        
        // Test contains
        assert!(hand.contains(card1));
        assert!(!hand.contains(Entity::from_raw(99)));
        
        // Test remaining capacity
        assert_eq!(hand.remaining_capacity(), 2);
    }
    
    #[test]
    fn test_score_operations() {
        let mut score = Score::new();
        
        // Add collected cards
        let cards = vec![Entity::from_raw(1), Entity::from_raw(2)];
        score.add_collected_cards(cards);
        assert_eq!(score.card_count(), 2);
        
        // Add more cards
        let more_cards = vec![Entity::from_raw(3), Entity::from_raw(4)];
        score.add_collected_cards(more_cards);
        assert_eq!(score.card_count(), 4);
        
        // Add kseri
        score.add_kseri();
        score.add_kseri();
        assert_eq!(score.kseri_count, 2);
        
        // Add double kseri
        score.add_double_kseri();
        assert_eq!(score.double_kseri_count, 1);
    }
}

#[cfg(test)]
mod table_pile_tests {
    use super::super::table::*;
    use bevy::prelude::*;
    
    #[test]
    fn test_table_pile_creation() {
        let pile = TablePile::new();
        assert_eq!(pile.count(), 0);
        assert!(pile.is_empty());
        assert!(pile.peek_top().is_none());
    }
    
    #[test]
    fn test_play_card() {
        let mut pile = TablePile::new();
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        let card3 = Entity::from_raw(3);
        
        // Play cards
        pile.play_card(card1);
        assert_eq!(pile.count(), 1);
        assert_eq!(pile.peek_top(), Some(card1));
        
        pile.play_card(card2);
        assert_eq!(pile.count(), 2);
        assert_eq!(pile.peek_top(), Some(card2));
        
        pile.add_card(card3); // Test alias
        assert_eq!(pile.count(), 3);
        assert_eq!(pile.peek_top(), Some(card3));
    }
    
    #[test]
    fn test_peek_and_top() {
        let mut pile = TablePile::new();
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        
        pile.play_card(card1);
        pile.play_card(card2);
        
        // Both methods should return the same
        assert_eq!(pile.peek_top(), Some(card2));
        assert_eq!(pile.top_card(), Some(card2));
        
        // Peek shouldn't remove the card
        assert_eq!(pile.count(), 2);
    }
    
    #[test]
    fn test_take_all() {
        let mut pile = TablePile::new();
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        let card3 = Entity::from_raw(3);
        
        pile.play_card(card1);
        pile.play_card(card2);
        pile.play_card(card3);
        
        let taken = pile.take_all();
        assert_eq!(taken.len(), 3);
        assert_eq!(taken[0], card1);
        assert_eq!(taken[1], card2);
        assert_eq!(taken[2], card3);
        
        // Pile should be empty after take_all
        assert!(pile.is_empty());
        assert_eq!(pile.count(), 0);
    }
    
    #[test]
    fn test_clear() {
        let mut pile = TablePile::new();
        pile.play_card(Entity::from_raw(1));
        pile.play_card(Entity::from_raw(2));
        
        pile.clear();
        assert!(pile.is_empty());
        assert_eq!(pile.count(), 0);
    }
    
    #[test]
    fn test_would_be_kseri() {
        let mut pile = TablePile::new();
        
        // Empty pile - not kseri
        assert!(!pile.would_be_kseri());
        
        // Single card - would be kseri
        pile.play_card(Entity::from_raw(1));
        assert!(pile.would_be_kseri());
        
        // Multiple cards - not kseri
        pile.play_card(Entity::from_raw(2));
        assert!(!pile.would_be_kseri());
    }
    
    #[test]
    fn test_iterator() {
        let mut pile = TablePile::new();
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        let card3 = Entity::from_raw(3);
        
        pile.play_card(card1);
        pile.play_card(card2);
        pile.play_card(card3);
        
        // Test iterator
        let cards: Vec<_> = pile.iter().copied().collect();
        assert_eq!(cards, vec![card1, card2, card3]);
        
        // Test get method
        assert_eq!(pile.get(0), Some(&card1));
        assert_eq!(pile.get(1), Some(&card2));
        assert_eq!(pile.get(2), Some(&card3));
        assert_eq!(pile.get(3), None);
    }
    
    #[test]
    fn test_into_iterator() {
        let mut pile = TablePile::new();
        let card1 = Entity::from_raw(1);
        let card2 = Entity::from_raw(2);
        
        pile.play_card(card1);
        pile.play_card(card2);
        
        // Test reference iterator
        let mut count = 0;
        for &card in &pile {
            count += 1;
            assert!(card == card1 || card == card2);
        }
        assert_eq!(count, 2);
        
        // Test consuming iterator
        let cards: Vec<_> = pile.into_iter().collect();
        assert_eq!(cards.len(), 2);
    }
}