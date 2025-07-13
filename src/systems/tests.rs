#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::*;
    use crate::systems::game_logic::*;
    use crate::systems::validation::*;

    #[test]
    fn test_game_manager_initialization() {
        let game_manager = GameManager::new(6);
        assert_eq!(game_manager.round_number, 0);
        assert_eq!(game_manager.total_rounds, 6);
        assert!(!game_manager.is_game_complete());
    }

    #[test]
    fn test_game_manager_round_progression() {
        let mut game_manager = GameManager::new(2);
        
        game_manager.start_new_round();
        assert_eq!(game_manager.round_number, 1);
        assert!(!game_manager.is_game_complete());
        
        game_manager.start_new_round();
        assert_eq!(game_manager.round_number, 2);
        assert!(game_manager.is_game_complete());
    }

    #[test]
    fn test_turn_manager_switching() {
        let mut turn_manager = TurnManager::default();
        assert_eq!(turn_manager.current_player, PlayerId::default());
        
        turn_manager.switch_turn();
        assert_eq!(turn_manager.current_player, PlayerId::PLAYER_TWO);
        assert_eq!(turn_manager.turn_number, 1);
        
        turn_manager.switch_turn();
        assert_eq!(turn_manager.current_player, PlayerId::PLAYER_ONE);
        assert_eq!(turn_manager.turn_number, 2);
    }

    #[test]
    fn test_validate_capture_rules_with_jack() {
        let jack = Card::new(Suit::Hearts, Rank::Jack);
        let table_cards = vec![
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Spades, Rank::Seven),
        ];
        
        let result = validate_capture_rules(jack, &table_cards);
        assert!(result.is_ok());
        let captured = result.unwrap();
        assert_eq!(captured.len(), 3);
    }

    #[test]
    fn test_validate_capture_rules_with_matching_rank() {
        let seven = Card::new(Suit::Hearts, Rank::Seven);
        let table_cards = vec![
            Card::new(Suit::Clubs, Rank::Seven),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Spades, Rank::Seven),
        ];
        
        let result = validate_capture_rules(seven, &table_cards);
        assert!(result.is_ok());
        let captured = result.unwrap();
        assert_eq!(captured.len(), 2);
        assert!(captured.iter().all(|c| c.rank == Rank::Seven));
    }

    #[test]
    fn test_validate_capture_rules_no_match() {
        let five = Card::new(Suit::Hearts, Rank::Five);
        let table_cards = vec![
            Card::new(Suit::Clubs, Rank::Seven),
            Card::new(Suit::Diamonds, Rank::King),
        ];
        
        let result = validate_capture_rules(five, &table_cards);
        assert!(result.is_ok());
        let captured = result.unwrap();
        assert_eq!(captured.len(), 0);
    }

    #[test]
    fn test_validate_kseri_valid() {
        let seven = Card::new(Suit::Hearts, Rank::Seven);
        let captured_seven = Card::new(Suit::Clubs, Rank::Seven);
        
        assert!(validate_kseri(seven, &[captured_seven], true));
    }

    #[test]
    fn test_validate_kseri_with_jack() {
        let jack = Card::new(Suit::Hearts, Rank::Jack);
        let captured_card = Card::new(Suit::Clubs, Rank::Seven);
        
        assert!(!validate_kseri(jack, &[captured_card], true));
    }

    #[test]
    fn test_validate_kseri_multiple_cards() {
        let seven = Card::new(Suit::Hearts, Rank::Seven);
        let captured_cards = vec![
            Card::new(Suit::Clubs, Rank::Seven),
            Card::new(Suit::Diamonds, Rank::Seven),
        ];
        
        assert!(!validate_kseri(seven, &captured_cards, false));
    }

    #[test]
    fn test_last_capture_tracking() {
        let last_capture = LastCapture {
            player_id: PlayerId::PLAYER_ONE,
            captured_cards: vec![
                Card::new(Suit::Hearts, Rank::King),
                Card::new(Suit::Clubs, Rank::King),
            ],
            was_kseri: false,
        };
        
        assert_eq!(last_capture.player_id, PlayerId::PLAYER_ONE);
        assert_eq!(last_capture.captured_cards.len(), 2);
        assert!(!last_capture.was_kseri);
    }

    #[test]
    fn test_round_state_initialization() {
        let round_state = RoundState::default();
        assert_eq!(round_state.cards_dealt, 0);
        assert!(round_state.initial_table_cards.is_empty());
        assert!(round_state.captures_this_round.is_empty());
    }

    #[test]
    fn test_animation_state() {
        let mut animation = AnimationState::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(10.0, 10.0, 0.0),
            1.0
        );
        
        assert_eq!(animation.progress, 0.0);
        assert_eq!(animation.current_position(), Vec3::ZERO);
        
        // Update halfway
        let done = animation.update(0.5);
        assert!(!done);
        assert_eq!(animation.progress, 0.5);
        assert_eq!(animation.current_position(), Vec3::new(5.0, 5.0, 0.0));
        
        // Complete animation
        let done = animation.update(0.6);
        assert!(done);
        assert_eq!(animation.current_position(), Vec3::new(10.0, 10.0, 0.0));
    }

    #[test]
    fn test_playing_phase_states() {
        // Test that all playing phases are defined
        let phases = [
            PlayingPhase::DealingCards,
            PlayingPhase::PlayerTurn,
            PlayingPhase::ProcessingCapture,
            PlayingPhase::RoundEnd,
        ];
        
        for (i, phase) in phases.iter().enumerate() {
            match phase {
                PlayingPhase::DealingCards => assert_eq!(i, 0),
                PlayingPhase::PlayerTurn => assert_eq!(i, 1),
                PlayingPhase::ProcessingCapture => assert_eq!(i, 2),
                PlayingPhase::RoundEnd => assert_eq!(i, 3),
            }
        }
    }
}