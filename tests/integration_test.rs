use bevy::prelude::*;
use kseri::game_plugin::{KseriGamePlugin, calculate_final_scores};
use kseri::systems::*;
use kseri::components::*;

#[test]
fn test_game_plugin_builds() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(KseriGamePlugin);
    
    // Verify resources are initialized
    assert!(app.world().contains_resource::<GameManager>());
    assert!(app.world().contains_resource::<TurnManager>());
    assert!(app.world().contains_resource::<RoundState>());
}

#[test]
fn test_dealing_system_integration() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_state::<GameState>();
    app.init_state::<PlayingPhase>();
    app.insert_resource(GameManager::new(6));
    app.init_resource::<TurnManager>();
    app.init_resource::<RoundState>();
    
    // Create test entities
    let deck_entity = app.world_mut().spawn((
        DeckComponent,
        Deck::new(),
        Transform::default(),
        GlobalTransform::default(),
    )).id();
    
    let table_entity = app.world_mut().spawn((
        TableComponent,
        TablePile::new(),
        Transform::default(),
        GlobalTransform::default(),
    )).id();
    
    app.world_mut().spawn((
        Player {
            id: PlayerId::PLAYER_ONE,
            name: "Test Player 1".to_string(),
            is_local: true,
        },
        Hand::new(4),
        Score::new(),
        Transform::default(),
        GlobalTransform::default(),
    ));
    
    app.world_mut().spawn((
        Player {
            id: PlayerId::PLAYER_TWO,
            name: "Test Player 2".to_string(),
            is_local: false,
        },
        Hand::new(4),
        Score::new(),
        Transform::default(),
        GlobalTransform::default(),
    ));
    
    // Set up game manager
    let mut game_manager = app.world_mut().resource_mut::<GameManager>();
    game_manager.deck_entity = Some(deck_entity);
    game_manager.table_entity = Some(table_entity);
    
    // Add dealing system
    app.add_systems(Update, deal_initial_cards);
    
    // Set to dealing phase
    app.world_mut().insert_resource(NextState::Pending(PlayingPhase::DealingCards));
    
    // Run the app for one frame
    app.update();
    
    // Verify cards were dealt
    let hand_query = app.world().query::<&Hand>().iter(app.world()).collect::<Vec<_>>();
    assert_eq!(hand_query.len(), 2);
    
    // Each player should have 4 cards after initial deal
    for hand in hand_query {
        assert_eq!(hand.count(), 4);
    }
}

#[test]
fn test_scoring_with_majority_bonus() {
    let mut app = App::new();
    
    // Create players with scores
    let player1 = app.world_mut().spawn((
        Player {
            id: PlayerId::PLAYER_ONE,
            name: "Player 1".to_string(),
            is_local: true,
        },
        Score::new(),
    )).id();
    
    let player2 = app.world_mut().spawn((
        Player {
            id: PlayerId::PLAYER_TWO,
            name: "Player 2".to_string(),
            is_local: false,
        },
        Score::new(),
    )).id();
    
    // Give player 1 majority of cards (27 cards)
    {
        let mut score = app.world_mut().get_mut::<Score>(player1).unwrap();
        for _ in 0..27 {
            score.add_collected_cards(vec![Entity::from_raw(1)]);
        }
    }
    
    // Give player 2 25 cards
    {
        let mut score = app.world_mut().get_mut::<Score>(player2).unwrap();
        for _ in 0..25 {
            score.add_collected_cards(vec![Entity::from_raw(2)]);
        }
    }
    
    // Run final scoring
    app.add_systems(Update, calculate_final_scores);
    app.update();
    
    // Player 1 should have gotten the majority bonus
    let score1 = app.world().get::<Score>(player1).unwrap();
    let score2 = app.world().get::<Score>(player2).unwrap();
    
    // Player 1 has 27 cards (>26) so gets +3 bonus
    assert_eq!(score1.total_points, 3);
    assert_eq!(score2.total_points, 0);
}