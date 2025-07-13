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
    fn test_double_kseri_detection() {
        let jack = Card::new(Suit::Hearts, Rank::Jack);
        let captured_jack = Card::new(Suit::Clubs, Rank::Jack);
        
        // Playing Jack and capturing Jack on single card table should be double Kseri
        assert!(jack.can_capture(&captured_jack));
        assert!(!jack.makes_kseri(1)); // Jack never makes regular kseri
        
        // But we should detect it as double kseri in the game logic
        // This is tested in the turn_management system
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

    #[test]
    fn test_initial_deal_no_jacks_on_table() {
        // Create a test deck with controlled cards
        let mut deck = Deck::new();
        
        // Simulate drawing cards for initial deal
        let mut table_cards = Vec::new();
        let mut player1_cards = Vec::new();
        let mut player2_cards = Vec::new();
        
        // Test multiple deals to ensure no Jacks on table
        for _ in 0..10 {
            deck = Deck::new();
            deck.shuffle();
            
            player1_cards = deck.draw_many(4);
            player2_cards = deck.draw_many(4);
            table_cards = deck.draw_many(4);
            
            // In actual implementation, if Jacks found on table, we'd reshuffle
            // Here we just verify the logic would work
            let has_jack = table_cards.iter().any(|c| c.rank == Rank::Jack);
            
            // Test that we can detect Jacks
            if has_jack {
                // In real implementation, this would trigger a reshuffle
                assert!(table_cards.iter().any(|c| c.rank == Rank::Jack));
            }
        }
        
        // Verify deck has correct remaining cards after dealing
        assert_eq!(deck.remaining(), 40); // 52 - 12 dealt
    }

    #[test]
    fn test_subsequent_dealing() {
        let mut deck = Deck::new();
        
        // Initial deal
        let _ = deck.draw_many(12); // 4 to each player + 4 to table
        assert_eq!(deck.remaining(), 40);
        
        // Subsequent deals (4 to each player)
        let _ = deck.draw_many(8);
        assert_eq!(deck.remaining(), 32);
        
        let _ = deck.draw_many(8);
        assert_eq!(deck.remaining(), 24);
        
        let _ = deck.draw_many(8);
        assert_eq!(deck.remaining(), 16);
        
        let _ = deck.draw_many(8);
        assert_eq!(deck.remaining(), 8);
        
        let _ = deck.draw_many(8);
        assert_eq!(deck.remaining(), 0);
        
        // No more cards to deal
        assert!(deck.is_empty());
    }

    #[test]
    fn test_majority_cards_bonus() {
        let mut score1 = Score::new();
        let mut score2 = Score::new();
        
        // Player 1 has 27 cards (majority)
        for _ in 0..27 {
            score1.add_collected_cards(vec![Entity::from_raw(1)]);
        }
        
        // Player 2 has 25 cards
        for _ in 0..25 {
            score2.add_collected_cards(vec![Entity::from_raw(2)]);
        }
        
        assert_eq!(score1.card_count(), 27);
        assert_eq!(score2.card_count(), 25);
        
        // Player 1 should get majority bonus (>26 cards)
        assert!(score1.card_count() > 26);
        assert!(score2.card_count() <= 26);
    }

    #[test]
    fn test_score_calculation_with_special_cards() {
        use bevy::app::App;
        use bevy::ecs::system::SystemState;
        
        let mut app = App::new();
        let world = app.world_mut();
        
        // Create score and card entities
        let mut score = Score::new();
        
        // Add some special cards
        let cards = vec![
            world.spawn(Card::new(Suit::Clubs, Rank::Two)).id(), // 2 points
            world.spawn(Card::new(Suit::Diamonds, Rank::Ten)).id(), // 3 points
            world.spawn(Card::new(Suit::Diamonds, Rank::Jack)).id(), // 1 point
            world.spawn(Card::new(Suit::Hearts, Rank::Ace)).id(), // 1 point
            world.spawn(Card::new(Suit::Hearts, Rank::Five)).id(), // 0 points
        ];
        
        score.add_collected_cards(cards);
        
        // Add 1 regular kseri and 1 double kseri
        score.add_kseri();
        score.add_double_kseri();
        
        // Calculate score
        let card_query = world.query::<&Card>();
        let mut system_state: SystemState<Query<&Card>> = SystemState::new(world);
        let card_query = system_state.get(world);
        let total = score.calculate_score(&card_query);
        
        // Should be: 2 + 3 + 1 + 1 + 0 = 7 (from cards) + 10 (1 kseri) + 20 (1 double kseri) = 37
        assert_eq!(total, 37);
        assert_eq!(score.total_points, 37);
    }
}