use bevy::prelude::*;
use crate::components::*;
use crate::systems::game_logic::*;
use rand::seq::SliceRandom;

/// Deal initial cards at the start of a round
pub fn deal_initial_cards(
    mut commands: Commands,
    mut deck_query: Query<&mut Deck, With<DeckComponent>>,
    mut hand_query: Query<(&Player, &mut Hand)>,
    mut table_query: Query<&mut TablePile, With<TableComponent>>,
    mut round_state: ResMut<RoundState>,
    game_manager: Res<GameManager>,
) {
    if round_state.cards_dealt > 0 {
        return; // Already dealt initial cards
    }

    let Ok(mut deck) = deck_query.single_mut() else {
        return;
    };

    // Keep trying until we get a valid initial deal (no Jacks on table)
    loop {
        // Reset deck if we need to reshuffle
        if deck.remaining() < 12 {
            *deck = Deck::new();
        }

        // Temporary storage for dealt cards
        let mut player_cards: Vec<(PlayerId, Vec<Card>)> = Vec::new();
        let mut table_cards: Vec<Card> = Vec::new();

        // Deal 4 cards to each player
        for (player, _) in hand_query.iter() {
            let cards = deck.draw_many(4);
            if cards.len() != 4 {
                // Not enough cards, reset and try again
                *deck = Deck::new();
                continue;
            }
            player_cards.push((player.id, cards));
        }

        // Deal 4 cards to table
        table_cards = deck.draw_many(4);
        if table_cards.len() != 4 {
            // Not enough cards, reset and try again
            *deck = Deck::new();
            continue;
        }

        // Check if any Jacks are on the table
        let has_jack_on_table = table_cards.iter().any(|card| card.rank == Rank::Jack);
        
        if has_jack_on_table {
            // Reshuffle everything back into deck and try again
            *deck = Deck::new();
            continue;
        }

        // Valid deal! Now actually create the card entities
        
        // Deal to players
        for (player_id, cards) in player_cards {
            for (player, mut hand) in hand_query.iter_mut() {
                if player.id == player_id {
                    for card in cards {
                        let card_entity = commands.spawn((
                            CardEntity {
                                card,
                                location: CardLocation::PlayerHand(player_id),
                                face_up: player.is_local, // Only show local player's cards
                            },
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            GlobalTransform::default(),
                            Selectable {
                                enabled: false,
                                player_id,
                            },
                        )).id();
                        
                        let _ = hand.add_card(card_entity);
                    }
                    break;
                }
            }
        }

        // Deal to table
        let Ok(mut table) = table_query.single_mut() else {
            return;
        };

        for (index, card) in table_cards.iter().enumerate() {
            let card_entity = commands.spawn((
                CardEntity {
                    card: *card,
                    location: CardLocation::Table,
                    face_up: true,
                },
                Transform::from_xyz(0.0, 0.0, 1.0 + index as f32 * 0.1),
                GlobalTransform::default(),
                TablePosition {
                    index,
                    position: Vec2::new(0.0, 0.0),
                },
            )).id();
            
            table.play_card(card_entity);
        }

        // Update round state
        round_state.cards_dealt = 12; // 4 to each player + 4 to table
        round_state.initial_table_cards = table_cards;

        break; // Successfully dealt
    }
}

/// Deal subsequent cards when hands are empty
pub fn deal_subsequent_cards(
    mut commands: Commands,
    mut deck_query: Query<&mut Deck, With<DeckComponent>>,
    mut hand_query: Query<(&Player, &mut Hand)>,
    mut round_state: ResMut<RoundState>,
    mut round_end_writer: EventWriter<RoundEndEvent>,
) {
    // Check if all hands are empty
    let all_hands_empty = hand_query.iter().all(|(_, hand)| hand.is_empty());
    if !all_hands_empty {
        return;
    }

    let Ok(mut deck) = deck_query.single_mut() else {
        return;
    };

    // Check if deck has enough cards
    if deck.remaining() < 8 {
        // Not enough cards for a full deal, trigger round end
        if deck.is_empty() {
            round_end_writer.send(RoundEndEvent {
                reason: RoundEndReason::DeckEmpty,
            });
        }
        return;
    }

    // Deal 4 cards to each player
    for (player, mut hand) in hand_query.iter_mut() {
        let cards = deck.draw_many(4);
        
        for card in cards {
            let card_entity = commands.spawn((
                CardEntity {
                    card,
                    location: CardLocation::PlayerHand(player.id),
                    face_up: player.is_local,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::default(),
                Selectable {
                    enabled: false,
                    player_id: player.id,
                },
            )).id();
            
            let _ = hand.add_card(card_entity);
        }
    }

    round_state.cards_dealt += 8;
}

/// System to check if dealing is needed
pub fn check_dealing_needed(
    playing_phase: Res<State<PlayingPhase>>,
    mut next_phase: ResMut<NextState<PlayingPhase>>,
    hand_query: Query<&Hand, With<Player>>,
    deck_query: Query<&Deck, With<DeckComponent>>,
    round_state: Res<RoundState>,
) {
    if *playing_phase.get() != PlayingPhase::PlayerTurn {
        return;
    }

    let all_hands_empty = hand_query.iter().all(|hand| hand.is_empty());
    
    if all_hands_empty {
        if let Ok(deck) = deck_query.single() {
            if deck.remaining() >= 8 {
                // Enough cards to deal another round
                next_phase.set(PlayingPhase::DealingCards);
            } else if round_state.cards_dealt > 0 {
                // Not enough cards and we've already dealt, end the round
                next_phase.set(PlayingPhase::RoundEnd);
            }
        }
    }
}