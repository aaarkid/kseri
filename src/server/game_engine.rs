use serde::{Deserialize, Serialize};

use crate::components::card::{Card, Rank, PlayerId};
#[cfg(test)]
use crate::components::card::Suit;
use crate::components::table::Deck;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KseriGameState {
    pub deck: Vec<Card>,
    pub player_hands: [Vec<Card>; 2],
    pub table_cards: Vec<Card>,
    pub player_scores: [Vec<Card>; 2],
    pub current_turn: PlayerId,
    pub last_capture_player: Option<PlayerId>,
    pub game_over: bool,
    pub kseri_count: [u32; 2],
}

impl KseriGameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let mut state = Self {
            deck: deck.cards,
            player_hands: [Vec::new(), Vec::new()],
            table_cards: Vec::new(),
            player_scores: [Vec::new(), Vec::new()],
            current_turn: PlayerId::PLAYER_ONE,
            last_capture_player: None,
            game_over: false,
            kseri_count: [0, 0],
        };
        
        // Deal initial cards
        state.deal_initial_cards();
        
        state
    }
    
    fn deal_initial_cards(&mut self) {
        // Deal 4 cards to table
        for _ in 0..4 {
            if let Some(card) = self.deck.pop() {
                self.table_cards.push(card);
            }
        }
        
        // Deal 4 cards to each player
        for _ in 0..4 {
            for player_idx in 0..2 {
                if let Some(card) = self.deck.pop() {
                    self.player_hands[player_idx].push(card);
                }
            }
        }
    }
    
    pub fn deal_new_hands(&mut self) -> bool {
        if self.deck.is_empty() {
            return false;
        }
        
        // Deal 4 cards to each player
        for _ in 0..4 {
            for player_idx in 0..2 {
                if let Some(card) = self.deck.pop() {
                    self.player_hands[player_idx].push(card);
                }
            }
        }
        
        true
    }
    
    pub fn play_card(&mut self, player: PlayerId, card: Card) -> Result<PlayCardResult, String> {
        // Validate it's the player's turn
        if self.current_turn != player {
            return Err("Not your turn".to_string());
        }
        
        // Validate player has the card
        let player_idx = player.0 as usize;
        let card_pos = self.player_hands[player_idx]
            .iter()
            .position(|c| c == &card)
            .ok_or("Card not in hand")?;
        
        // Remove card from hand
        self.player_hands[player_idx].remove(card_pos);
        
        // Check for capture
        let mut captured_cards = Vec::new();
        let mut is_kseri = false;
        
        if !self.table_cards.is_empty() {
            let should_capture = if card.rank == Rank::Jack {
                // Jacks capture everything
                true
            } else {
                // Check if the played card matches the top card
                self.table_cards.last()
                    .map(|top| top.rank == card.rank)
                    .unwrap_or(false)
            };
            
            if should_capture {
                // Capture all table cards
                captured_cards = std::mem::take(&mut self.table_cards);
                
                // Check for Kseri
                if captured_cards.len() == 1 && card.rank != Rank::Jack && captured_cards[0].rank == card.rank {
                    is_kseri = true;
                    self.kseri_count[player_idx] += 1;
                }
                
                // Add played card to captured cards
                captured_cards.push(card);
                
                // Add to player's score pile
                self.player_scores[player_idx].extend(captured_cards.clone());
                
                self.last_capture_player = Some(player);
            } else {
                // No capture - add card to table
                self.table_cards.push(card);
            }
        } else {
            // Table is empty - just place the card
            self.table_cards.push(card);
        }
        
        // Switch turns
        self.current_turn = if player == PlayerId::PLAYER_ONE { PlayerId::PLAYER_TWO } else { PlayerId::PLAYER_ONE };
        
        // Check if hands are empty
        if self.player_hands[0].is_empty() && self.player_hands[1].is_empty() {
            if !self.deal_new_hands() {
                // No more cards to deal - game ending
                self.handle_game_end();
            }
        }
        
        Ok(PlayCardResult {
            captured_cards,
            is_kseri,
            game_ended: self.game_over,
        })
    }
    
    fn handle_game_end(&mut self) {
        self.game_over = true;
        
        // Award remaining table cards to last capturer
        if !self.table_cards.is_empty() {
            if let Some(last_player) = self.last_capture_player {
                let remaining = std::mem::take(&mut self.table_cards);
                self.player_scores[last_player.0 as usize].extend(remaining);
            }
        }
    }
    
    pub fn calculate_scores(&self) -> [u32; 2] {
        let mut scores = [0u32; 2];
        
        for (player_idx, captured) in self.player_scores.iter().enumerate() {
            for card in captured {
                scores[player_idx] += card.kseri_value();
            }
            
            // Add Kseri bonuses (10 points each)
            scores[player_idx] += self.kseri_count[player_idx] * 10;
        }
        
        // Award 3 points for majority of cards
        let card_counts = [
            self.player_scores[0].len(),
            self.player_scores[1].len(),
        ];
        
        if card_counts[0] > card_counts[1] {
            scores[0] += 3;
        } else if card_counts[1] > card_counts[0] {
            scores[1] += 3;
        }
        
        scores
    }
    
    pub fn get_winner(&self) -> Option<PlayerId> {
        if !self.game_over {
            return None;
        }
        
        let scores = self.calculate_scores();
        if scores[0] > scores[1] {
            Some(PlayerId::PLAYER_ONE)
        } else if scores[1] > scores[0] {
            Some(PlayerId::PLAYER_TWO)
        } else {
            None // Tie
        }
    }
    
    pub fn validate_state(&self) -> Result<(), String> {
        // Validate total cards
        let total_cards = self.deck.len() 
            + self.player_hands[0].len() 
            + self.player_hands[1].len()
            + self.table_cards.len()
            + self.player_scores[0].len()
            + self.player_scores[1].len();
            
        if total_cards != 52 {
            return Err(format!("Invalid card count: {}", total_cards));
        }
        
        // Check for duplicate cards
        let mut all_cards: Vec<Card> = Vec::new();
        all_cards.extend(self.deck.clone());
        all_cards.extend(self.player_hands[0].clone());
        all_cards.extend(self.player_hands[1].clone());
        all_cards.extend(self.table_cards.clone());
        all_cards.extend(self.player_scores[0].clone());
        all_cards.extend(self.player_scores[1].clone());
        
        all_cards.sort();
        let original_len = all_cards.len();
        all_cards.dedup();
        
        if all_cards.len() != original_len {
            return Err("Duplicate cards found".to_string());
        }
        
        Ok(())
    }
}

pub struct PlayCardResult {
    pub captured_cards: Vec<Card>,
    pub is_kseri: bool,
    pub game_ended: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_game_state() {
        let state = KseriGameState::new();
        
        // Check initial deal
        assert_eq!(state.deck.len(), 40); // 52 - 4 (table) - 4 (P1) - 4 (P2)
        assert_eq!(state.table_cards.len(), 4);
        assert_eq!(state.player_hands[0].len(), 4);
        assert_eq!(state.player_hands[1].len(), 4);
        assert_eq!(state.player_scores[0].len(), 0);
        assert_eq!(state.player_scores[1].len(), 0);
        assert_eq!(state.current_turn, PlayerId::PLAYER_ONE);
        assert!(!state.game_over);
        
        // Validate state
        assert!(state.validate_state().is_ok());
    }
    
    #[test]
    fn test_play_card_capture() {
        let mut state = KseriGameState::new();
        
        // Set up a known table state
        state.table_cards = vec![Card { suit: Suit::Hearts, rank: Rank::Seven }];
        state.player_hands[0] = vec![
            Card { suit: Suit::Diamonds, rank: Rank::Seven },
            Card { suit: Suit::Clubs, rank: Rank::Ace },
        ];
        
        // Play matching card
        let result = state.play_card(PlayerId::PLAYER_ONE, Card { suit: Suit::Diamonds, rank: Rank::Seven }).unwrap();
        
        assert_eq!(result.captured_cards.len(), 2); // Table card + played card
        assert!(result.is_kseri); // Single card capture with matching rank
        assert_eq!(state.table_cards.len(), 0);
        assert_eq!(state.player_scores[0].len(), 2);
        assert_eq!(state.current_turn, PlayerId::PLAYER_TWO);
    }
    
    #[test]
    fn test_jack_captures_all() {
        let mut state = KseriGameState::new();
        
        // Set up table with multiple cards
        state.table_cards = vec![
            Card { suit: Suit::Hearts, rank: Rank::Seven },
            Card { suit: Suit::Clubs, rank: Rank::King },
            Card { suit: Suit::Diamonds, rank: Rank::Three },
        ];
        state.player_hands[0] = vec![
            Card { suit: Suit::Spades, rank: Rank::Jack },
        ];
        
        // Play Jack
        let result = state.play_card(PlayerId::PLAYER_ONE, Card { suit: Suit::Spades, rank: Rank::Jack }).unwrap();
        
        assert_eq!(result.captured_cards.len(), 4); // 3 table cards + Jack
        assert!(!result.is_kseri); // Jack captures don't count as Kseri
        assert_eq!(state.table_cards.len(), 0);
        assert_eq!(state.player_scores[0].len(), 4);
    }
}