use super::*;

#[test]
fn test_value_unter() {
    let card = Card::new(Suit::Hearts, Rank::Unter);
    assert_eq!(2, value(card));
}

#[test]
fn test_value_ober() {
    let card = Card::new(Suit::Bells, Rank::Ober);
    assert_eq!(3, value(card));
}

#[test]
fn test_value_king() {
    let card = Card::new(Suit::Leaves, Rank::King);
    assert_eq!(4, value(card));
}

#[test]
fn test_value_ten() {
    let card = Card::new(Suit::Hearts, Rank::Ten);
    assert_eq!(10, value(card));
}

#[test]
fn test_value_ace() {
    let card = Card::new(Suit::Acorns, Rank::Ace);
    assert_eq!(11, value(card));
}

#[test]
fn test_value_illegal_card() {
    let card = Card::new(Suit::Acorns, Rank::Seven);
    assert_eq!(0, value(card));
}

#[test]
fn test_generate_deck() {
    const EXPECTED_CARDS: [Card; 20] = [
        Card{suit: Suit::Hearts, rank: Rank::Unter},
        Card{suit: Suit::Hearts, rank: Rank::Ober},
        Card{suit: Suit::Hearts, rank: Rank::King},
        Card{suit: Suit::Hearts, rank: Rank::Ten},
        Card{suit: Suit::Hearts, rank: Rank::Ace},

        Card{suit: Suit::Bells, rank: Rank::Unter},
        Card{suit: Suit::Bells, rank: Rank::Ober},
        Card{suit: Suit::Bells, rank: Rank::King},
        Card{suit: Suit::Bells, rank: Rank::Ten},
        Card{suit: Suit::Bells, rank: Rank::Ace},
        
        Card{suit: Suit::Acorns, rank: Rank::Unter},
        Card{suit: Suit::Acorns, rank: Rank::Ober},
        Card{suit: Suit::Acorns, rank: Rank::King},
        Card{suit: Suit::Acorns, rank: Rank::Ten},
        Card{suit: Suit::Acorns, rank: Rank::Ace},
        
        Card{suit: Suit::Leaves, rank: Rank::Unter},
        Card{suit: Suit::Leaves, rank: Rank::Ober},
        Card{suit: Suit::Leaves, rank: Rank::King},
        Card{suit: Suit::Leaves, rank: Rank::Ten},
        Card{suit: Suit::Leaves, rank: Rank::Ace},
    ];

    let actual_cards = generate_deck();

    assert_eq!(EXPECTED_CARDS.len(), actual_cards.len());
    
    for card in EXPECTED_CARDS.iter() {
        assert!(actual_cards.contains(card));
    }
}

#[test]
fn test_first_beats_second_same_suit() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ace};
    let card2 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let trump = Suit::Bells;

    assert!(first_beats_second(card1, card2, trump));
    assert!(!first_beats_second(card2, card1, trump));
}

#[test]
fn test_first_beats_second_same_suit_trump() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ace};
    let card2 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let trump = Suit::Leaves;

    assert!(first_beats_second(card1, card2, trump));
    assert!(!first_beats_second(card2, card1, trump));
}

#[test]
fn test_first_beats_second_different_suit() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let card2 = Card {suit: Suit::Bells, rank: Rank::Ace};
    let trump = Suit::Hearts;

    assert!(first_beats_second(card1, card2, trump));
    assert!(first_beats_second(card2, card1, trump));
}


#[test]
fn test_first_beats_second_different_suit_trump() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let card2 = Card {suit: Suit::Bells, rank: Rank::King};
    let trump = Suit::Bells;

    assert!(!first_beats_second(card1, card2, trump));
    assert!(first_beats_second(card2, card1, trump));
}

#[test]
fn test_player_score() {
    let wins = vec![Card {suit: Suit::Leaves, rank: Rank::Ten},
                    Card {suit: Suit::Bells, rank: Rank::Ace},
                    Card {suit: Suit::Bells, rank: Rank::King},
                    Card {suit: Suit::Bells, rank: Rank::Ober}];

    let player = Player {name: "Player".to_string(), wins,
                         twenties: vec![Suit::Bells, Suit::Hearts],
                         forty: Some(Suit::Leaves),
                         ..Default::default()};

    assert_eq!(28 + 2 * 20 + 40, player.score());
}

#[test]
fn test_new_random_game() {
    let game = Game::new_random();

    assert_eq!(10, game.stock.len());
    assert_eq!(5, game.player1.hand.len());
    assert_eq!(5, game.player2.hand.len());

    assert_eq!(game.stock[0].suit(), game.trump);
}

#[test]
fn test_trump_card_available() {
    let game = Game::default();

    assert_eq!(Some(game.stock[0]), game.trump_card());
}

#[test]
fn test_trump_card_closed() {
    let game = Game {closed: true, ..Default::default()};

    assert_eq!(None, game.trump_card());
}

#[test]
fn test_trump_card_no_card_left() {
    let game = Game {
        stock: Vec::new(), ..Default::default()
    };

    assert_eq!(None, game.trump_card());
}

#[test]
fn test_close_ok() {
    let mut game = Game::default();

    assert!(!game.is_closed());

    assert!(game.can_close());
    let result = game.close();

    assert!(result);
    assert!(game.is_closed());
}

#[test]
fn test_close_already_closed() {
    let mut game = Game::default();

    assert!(!game.is_closed());

    game.close();
    assert!(game.is_closed());

    assert!(!game.can_close());
    
    let result = game.close();
    assert!(!result);
}

#[test]
fn test_close_not_enough_cards_left() {
    let mut game = Game {
        stock: vec![Card::new(Suit::Leaves, Rank::Ace),
                   Card::new(Suit::Leaves, Rank::Ten)],
        trump: Suit::Leaves,
        ..Default::default()
    };

    assert!(!game.is_closed());

    assert!(!game.can_close());
    let result = game.close();

    assert!(!result);
    assert!(!game.is_closed());
}

#[test]
fn test_exchange_trump_not_having_trump_unter() {
    let stock = vec![Card{suit: Suit::Hearts, rank: Rank::Ober},
                    Card{suit: Suit::Hearts, rank: Rank::King},
                    Card{suit: Suit::Hearts, rank: Rank::Ten},
                    Card{suit: Suit::Hearts, rank: Rank::Ace}];

    let player1_hand = vec![Card{suit: Suit::Bells, rank: Rank::Unter},
                            Card{suit: Suit::Bells, rank: Rank::Ober},
                            Card{suit: Suit::Bells, rank: Rank::King},
                            Card{suit: Suit::Bells, rank: Rank::Ten},
                            Card{suit: Suit::Bells, rank: Rank::Ace}];
        
    let player2_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Unter},
                            Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::Ace}];

    let trump_card = stock[0];
    let trump = trump_card.suit();

    let player1 = Player {hand: player1_hand, ..Default::default()};
    let player2 = Player {hand: player2_hand, ..Default::default()};

    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

    assert!(!game.can_exchange_trump());

    let result = game.exchange_trump();
    assert!(!result);

    assert_eq!(trump_card, game.trump_card().unwrap());
}

#[test]
fn test_exchange_trump_not_enough_cards_in_stock() {
    let stock = vec![Card{suit: Suit::Hearts, rank: Rank::Ober},
                    Card{suit: Suit::Hearts, rank: Rank::King}];

    let player1_hand = vec![Card{suit: Suit::Hearts, rank: Rank::Unter},
                            Card{suit: Suit::Bells, rank: Rank::Ober},
                            Card{suit: Suit::Bells, rank: Rank::King},
                            Card{suit: Suit::Bells, rank: Rank::Ten},
                            Card{suit: Suit::Bells, rank: Rank::Ace}];
        
    let player2_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Unter},
                            Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::Ace}];

    let trump_card = stock[0];
    let trump = trump_card.suit();

    let player1 = Player {hand: player1_hand, ..Default::default()};
    let player2 = Player {hand: player2_hand, ..Default::default()};

    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

    assert!(!game.can_exchange_trump());

    let result = game.exchange_trump();
    assert!(!result);

    assert_eq!(trump_card, game.trump_card().unwrap());
}

#[test]
fn test_exchange_trump_closed() {
     let stock = vec![Card{suit: Suit::Hearts, rank: Rank::Ober},
                    Card{suit: Suit::Hearts, rank: Rank::King},
                    Card{suit: Suit::Hearts, rank: Rank::Ten},
                    Card{suit: Suit::Hearts, rank: Rank::Ace}];

    let player1_hand = vec![Card{suit: Suit::Hearts, rank: Rank::Unter},
                            Card{suit: Suit::Bells, rank: Rank::Ober},
                            Card{suit: Suit::Bells, rank: Rank::King},
                            Card{suit: Suit::Bells, rank: Rank::Ten},
                            Card{suit: Suit::Bells, rank: Rank::Ace}];
        
    let player2_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Unter},
                            Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::Ace}];

    let trump_card = stock[0];
    let trump = trump_card.suit();

    let player1 = Player {hand: player1_hand, ..Default::default()};
    let player2 = Player {hand: player2_hand, ..Default::default()};

    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

    game.close();
    assert!(game.is_closed());
    
    assert!(!game.can_exchange_trump());

    let result = game.exchange_trump();
    assert!(!result);

    assert_eq!(None, game.trump_card());
    assert!(game.player1.hand.contains(&Card::new(trump, Rank::Unter)));
}

#[test]
fn exchange_trump_ok() {
     let stock = vec![Card{suit: Suit::Hearts, rank: Rank::Ober},
                    Card{suit: Suit::Hearts, rank: Rank::King},
                    Card{suit: Suit::Hearts, rank: Rank::Ten},
                    Card{suit: Suit::Hearts, rank: Rank::Ace}];

    let player1_hand = vec![Card{suit: Suit::Hearts, rank: Rank::Unter},
                            Card{suit: Suit::Bells, rank: Rank::Ober},
                            Card{suit: Suit::Bells, rank: Rank::King},
                            Card{suit: Suit::Bells, rank: Rank::Ten},
                            Card{suit: Suit::Bells, rank: Rank::Ace}];
        
    let player2_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Unter},
                            Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::Ace}];

    let trump_card = stock[0];
    let trump = trump_card.suit();

    let player1 = Player {hand: player1_hand, ..Default::default()};
    let player2 = Player {hand: player2_hand, ..Default::default()};

    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

    assert!(game.can_exchange_trump());

    let result = game.exchange_trump();
    assert!(result);

    assert_eq!(Card::new(trump, Rank::Unter), game.trump_card().unwrap());
    assert!(game.player1.hand.contains(&trump_card));
}

#[test]
fn test_call_twenty_no_such_cards() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Ten),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};
    
    let mut game = Game {stock, trump: Suit::Hearts,
                         player1, player2, ..Default::default()};

    assert!(!game.can_call_twenty(Suit::Bells));

    let result = game.call_twenty(Suit::Bells);
    assert!(!result);
}

#[test]
fn test_call_twenty_already_called_the_same() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};
    
    let mut game = Game {stock, trump: Suit::Hearts,
                         player1, player2, ..Default::default()};

    let twenty_suit = Suit::Bells;
    
    game.call_twenty(twenty_suit);
    assert!(game.player1.twenties.contains(&twenty_suit));

    assert!(!game.can_call_twenty(twenty_suit));

    let result = game.call_twenty(twenty_suit);
    assert!(!result);
}

#[test]
fn test_call_twenty_suit_is_trump() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let twenty_suit = Suit::Bells;
    
    let mut game = Game {stock, trump: twenty_suit, player1,
                         player2, ..Default::default()};

    assert!(!game.can_call_twenty(twenty_suit));

    let result = game.call_twenty(twenty_suit);
    assert!(!result);

    assert!(!game.player1.twenties.contains(&twenty_suit));
}

#[test]
fn test_call_twenty_ok() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};
    
    let mut game = Game {stock, player1, player2, ..Default::default()};

    let twenty_suit = Suit::Bells;
    assert!(game.can_call_twenty(twenty_suit));

    let result = game.call_twenty(twenty_suit);
    assert!(result);

    assert!(game.player1.twenties.contains(&twenty_suit));
}

#[test]
fn test_call_forty_no_such_cards() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Ten),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Bells;
    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

    assert!(!game.can_call_forty());

    let result = game.call_forty();
    assert!(!result);
}

#[test]
fn test_call_forty_already_called_the_same() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Bells;
    let mut game = Game {stock, trump, player1, player2, ..Default::default()};
    
    game.call_forty();
    assert_eq!(Some(trump), game.player1.forty);

    assert!(!game.can_call_forty());

    let result = game.call_forty();
    assert!(!result);
}

#[test]
fn test_call_forty_ok() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Bells;
    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

    assert!(game.can_call_forty());

    let result = game.call_forty();
    assert!(result);

    assert_eq!(Some(trump), game.player1.forty);
}

#[test]
fn declare_win_already_game_over() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten),
                            Card::new(Suit::Bells, Rank::Ace),
                            Card::new(Suit::Bells, Rank::Ten),
                            Card::new(Suit::Hearts, Rank::Ace),
                            Card::new(Suit::Hearts, Rank::Ten),
                            Card::new(Suit::Leaves, Rank::King),
                            Card::new(Suit::Leaves, Rank::Ober)];
    let player1 = Player {wins: player1_wins, ..Default::default()};
    let mut game = Game {player1, ..Default::default()};

    game.declare_win();
    assert!(game.is_game_over());
    
    assert!(!game.can_declare_win());

    let result = game.declare_win();
    assert!(!result);
}

#[test]
fn declare_win_not_enough() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten)];
    let player1 = Player {wins: player1_wins, ..Default::default()};
    let mut game = Game {player1, ..Default::default()};

    assert!(!game.can_declare_win());

    let result = game.declare_win();
    assert!(!result);
}

#[test]
fn declare_win_ok() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten),
                            Card::new(Suit::Bells, Rank::Ace),
                            Card::new(Suit::Bells, Rank::Ten),
                            Card::new(Suit::Hearts, Rank::Ace),
                            Card::new(Suit::Hearts, Rank::Ten),
                            Card::new(Suit::Leaves, Rank::King),
                            Card::new(Suit::Leaves, Rank::Ober)];
    let player1 = Player {wins: player1_wins, ..Default::default()};
    let mut game = Game {player1, ..Default::default()};

    assert!(game.can_declare_win());

    let result = game.declare_win();
    assert!(result);
}

#[test]
fn test_next_turn_invalid_player1_card() {
    let mut game = Game::default();

    let removed_card = game.player1.hand.pop().unwrap();
    let player2_card = *game.player2.hand.first().unwrap();

    let result = game.next_turn(removed_card, player2_card);
    assert!(!result);
}

#[test]
fn test_next_turn_invalid_player2_card() {
    let mut game = Game::default();

    let removed_card = game.player2.hand.pop().unwrap();
    let player1_card = *game.player1.hand.first().unwrap();
    
    let result = game.next_turn(player1_card, removed_card);
    assert!(!result);
}

#[test]
fn test_next_turn_illegal_suit() {
    let stock = vec![Card{suit: Suit::Hearts, rank: Rank::Unter},
                    Card{suit: Suit::Hearts, rank: Rank::Ober},
                    Card{suit: Suit::Hearts, rank: Rank::King},
                    Card{suit: Suit::Hearts, rank: Rank::Ten},
                    Card{suit: Suit::Hearts, rank: Rank::Ace},
                    Card{suit: Suit::Acorns, rank: Rank::Unter}];

    let trump = stock[0].suit();

    let player1_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::Ace},
                            Card{suit: Suit::Leaves, rank: Rank::Unter},
                            Card{suit: Suit::Leaves, rank: Rank::Ober},
                            Card{suit: Suit::Leaves, rank: Rank::King}];

    let player2_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Leaves, rank: Rank::Ten},
                            Card{suit: Suit::Leaves, rank: Rank::Ace},
                            Card{suit: Suit::Bells, rank: Rank::Unter}];

    let player1_card = player1_hand[0];
    let player2_card = player2_hand[2];
    
    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

    let closing_successful = game.close();

    assert!(closing_successful);
    assert!(game.is_closed());

    let result = game.next_turn(player1_card, player2_card);
    assert!(!result);
}

#[test]
fn test_next_turn_closed_must_be_higher() {
    let stock = Vec::new();

    let trump = Suit::Hearts;

    let player1_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Leaves, rank: Rank::Unter},
                            Card{suit: Suit::Leaves, rank: Rank::Ober},
                            Card{suit: Suit::Leaves, rank: Rank::King}];

    let player2_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::Ace},
                            Card{suit: Suit::Leaves, rank: Rank::Ten},
                            Card{suit: Suit::Leaves, rank: Rank::Ace},
                            Card{suit: Suit::Bells, rank: Rank::Unter}];

    let player1_card = player1_hand[0];
    let player2_card = player2_hand[0];
    
    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

    let result = game.next_turn(player1_card, player2_card);
    assert!(!result);
}

#[test]
fn test_next_turn_closed_must_use_trump() {
    let stock = Vec::new();

    let trump = Suit::Hearts;

    let player1_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Leaves, rank: Rank::Unter},
                            Card{suit: Suit::Leaves, rank: Rank::Ober},
                            Card{suit: Suit::Leaves, rank: Rank::King}];

    let player2_hand = vec![Card{suit: Suit::Bells, rank: Rank::Ober},
                            Card{suit: Suit::Bells, rank: Rank::Ace},
                            Card{suit: Suit::Leaves, rank: Rank::Ten},
                            Card{suit: Suit::Leaves, rank: Rank::Ace},
                            Card{suit: Suit::Hearts, rank: Rank::Unter}];

    let player1_card = player1_hand[0];
    let player2_card = player2_hand[0];
    
    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

    let result = game.next_turn(player1_card, player2_card);
    assert!(!result);
}

#[test]
fn test_next_turn_ok() {
    let stock = vec![
        Card{suit: Suit::Hearts, rank: Rank::Unter},
        Card{suit: Suit::Hearts, rank: Rank::Ober},
        Card{suit: Suit::Hearts, rank: Rank::King},
        Card{suit: Suit::Hearts, rank: Rank::Ten},
        Card{suit: Suit::Hearts, rank: Rank::Ace},
        
        Card{suit: Suit::Acorns, rank: Rank::Unter},
        Card{suit: Suit::Acorns, rank: Rank::Ober},
        Card{suit: Suit::Acorns, rank: Rank::King},
        Card{suit: Suit::Acorns, rank: Rank::Ten},
        Card{suit: Suit::Acorns, rank: Rank::Ace},
    ];

    let trump = stock[0].suit();

    let player1_hand = vec![
        Card{suit: Suit::Bells, rank: Rank::Unter},
        Card{suit: Suit::Bells, rank: Rank::Ober},
        Card{suit: Suit::Bells, rank: Rank::King},
        Card{suit: Suit::Leaves, rank: Rank::Ace},
        Card{suit: Suit::Bells, rank: Rank::Ten},
    ];

    let player2_hand = vec![
        Card{suit: Suit::Leaves, rank: Rank::Unter},
        Card{suit: Suit::Leaves, rank: Rank::Ober},
        Card{suit: Suit::Leaves, rank: Rank::King},
        Card{suit: Suit::Leaves, rank: Rank::Ten},
        Card{suit: Suit::Bells, rank: Rank::Ace},
    ];

    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};
    
    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

    let card1 = game.player1.hand[4];
    let card2 = game.player2.hand[4];
    let result = game.next_turn(card1, card2);
    assert!(result);

    assert!(!game.player1.hand.contains(&card1));
    assert!(!game.player2.hand.contains(&card2));

    assert_eq!(5, game.player1.hand.len());
    assert_eq!(5, game.player2.hand.len());

    assert_eq!(8, game.stock.len());
    
    assert!(game.player2.wins.contains(&card1));
    assert!(game.player2.wins.contains(&card2));
    assert_eq!(game.player2.name, game.next_player_name());
}
