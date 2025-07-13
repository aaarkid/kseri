use bevy::prelude::*;
use crate::components::*;
use crate::systems::game_logic::*;

#[derive(Debug, Clone)]
pub enum CaptureError {
    NoMatch,
}

pub fn validate_capture_rules(played_card: Card, table_cards: &[Card]) -> Result<Vec<Card>, CaptureError> {
    let mut captured = Vec::new();
    
    if played_card.rank == Rank::Jack {
        // Jack captures all cards
        captured.extend_from_slice(table_cards);
    } else {
        // Regular cards capture only matching ranks
        for card in table_cards {
            if card.rank == played_card.rank {
                captured.push(*card);
            }
        }
    }
    
    Ok(captured)
}

pub fn validate_kseri(played_card: Card, captured_cards: &[Card], was_single_card: bool) -> bool {
    // Kseri happens when:
    // 1. Exactly one card was captured
    // 2. The played card is not a Jack
    // 3. The captured card matches the played card's rank
    // 4. There was only a single card on the table
    captured_cards.len() == 1 
        && was_single_card
        && played_card.rank != Rank::Jack 
        && captured_cards[0].rank == played_card.rank
}

pub fn validate_play_system(
    turn_manager: Res<TurnManager>,
    mut action_events: EventReader<PlayerActionEvent>,
    hand_query: Query<(&Player, &Hand)>,
    card_entity_query: Query<&CardEntity>,
) -> bool {
    for event in action_events.read() {
        // Check if it's the correct player's turn
        if event.player_id != turn_manager.current_player {
            // warn!("Invalid play: Not player {:?}'s turn", event.player_id);
            return false;
        }
        
        match &event.action {
            PlayerAction::PlayCard(entity) => {
                // Check if the card exists
                let Ok(_card_entity) = card_entity_query.get(*entity) else {
                    // warn!("Invalid play: Card entity not found");
                    return false;
                };
                
                // Check if the card is in the player's hand
                for (player, hand) in hand_query.iter() {
                    if player.id == event.player_id {
                        // Check if the card entity is in the player's hand
                        if !hand.contains(*entity) {
                            // warn!("Invalid play: Card not in player's hand");
                            return false;
                        }
                        break;
                    }
                }
            }
        }
    }
    
    true
}

pub fn validate_state_system(
    game_state: Res<State<GameState>>,
    game_manager: Res<GameManager>,
    deck_query: Query<&Deck, With<DeckComponent>>,
    hand_query: Query<&Hand, With<Player>>,
    table_query: Query<&TablePile, With<TableComponent>>,
    score_query: Query<&Score, With<Player>>,
) {
    // Only validate during playing state
    if *game_state.get() != GameState::Playing {
        return;
    }
    
    // Count total cards in the game
    let mut total_cards = 0;
    
    // Cards in deck
    if let Ok(deck) = deck_query.single() {
        total_cards += deck.remaining();
    }
    
    // Cards in hands
    for hand in hand_query.iter() {
        total_cards += hand.count();
    }
    
    // Cards on table
    if let Ok(table) = table_query.single() {
        total_cards += table.cards.len();
    }
    
    // Cards in score piles
    for score in score_query.iter() {
        total_cards += score.cards_collected.len();
    }
    
    // Validate total cards (should always be 52)
    if total_cards != 52 {
        // error!("State validation failed: Total cards = {}, expected 52", total_cards);
    }
    
    // Validate hands don't exceed capacity
    for hand in hand_query.iter() {
        if hand.count() > 4 {
            // error!("State validation failed: Hand has {} cards, max is 4", hand.count());
        }
    }
    
    // Validate round number
    if game_manager.round_number > game_manager.total_rounds {
        // error!("State validation failed: Round {} exceeds total rounds {}", 
        //     game_manager.round_number, game_manager.total_rounds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_capture_rules() {
        // Test Jack captures all
        let jack = Card::new(Suit::Hearts, Rank::Jack);
        let table = vec![
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::King),
        ];
        let captured = validate_capture_rules(jack, &table).unwrap();
        assert_eq!(captured.len(), 2);
        
        // Test regular capture
        let seven = Card::new(Suit::Hearts, Rank::Seven);
        let table = vec![
            Card::new(Suit::Clubs, Rank::Seven),
            Card::new(Suit::Diamonds, Rank::King),
        ];
        let captured = validate_capture_rules(seven, &table).unwrap();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].rank, Rank::Seven);
        
        // Test no capture
        let five = Card::new(Suit::Hearts, Rank::Five);
        let table = vec![
            Card::new(Suit::Clubs, Rank::Seven),
            Card::new(Suit::Diamonds, Rank::King),
        ];
        let captured = validate_capture_rules(five, &table).unwrap();
        assert_eq!(captured.len(), 0);
    }
    
    #[test]
    fn test_validate_kseri() {
        let seven = Card::new(Suit::Hearts, Rank::Seven);
        let captured_seven = Card::new(Suit::Clubs, Rank::Seven);
        
        // Valid Kseri
        assert!(validate_kseri(seven, &[captured_seven], true));
        
        // Not Kseri - Jack used
        let jack = Card::new(Suit::Hearts, Rank::Jack);
        assert!(!validate_kseri(jack, &[captured_seven], true));
        
        // Not Kseri - multiple cards captured
        let two_cards = vec![captured_seven, Card::new(Suit::Diamonds, Rank::Seven)];
        assert!(!validate_kseri(seven, &two_cards, false));
    }
}