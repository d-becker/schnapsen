use super::*;

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

    let player = game.player_on_turn();
    
    assert!(!game.is_closed());

    assert!(game.can_close(player).is_ok());
    let result = game.close(player);

    assert!(result.is_ok());
    assert!(game.is_closed());
}

#[test]
fn test_close_player_not_on_turn() {
    let mut game = Game::default();

    let player2_marker = game.player_on_turn().other();
    
    assert!(!game.is_closed());

    let expected_error = Err(ErrorKind::NotPlayersTurn);
    
    assert!(game.can_close(player2_marker).is_err());
    let result = game.close(player2_marker);

    assert_eq!(expected_error, result);
    assert!(!game.is_closed());
}

#[test]
fn test_close_player_not_on_lead() {
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

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    let card1 = game.player1.hand[0];

    let first_card_result = game.play_card(player1_marker, card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_close(player2_marker));

    let result = game.close(player2_marker);
    assert_eq!(expected_error, result);
}

#[test]
fn test_close_already_closed() {
    let mut game = Game::default();
    let player1_marker = game.player_on_turn();

    assert!(!game.is_closed());

    let close_result = game.close(player1_marker);
    assert!(close_result.is_ok());
    assert!(game.is_closed());

    assert!(game.can_close(player1_marker).is_err());
    
    let result = game.close(player1_marker);
    assert_eq!(Err(ErrorKind::DeckClosed), result);
}

#[test]
fn test_close_not_enough_cards_left() {
    let mut game = Game {
        stock: vec![Card::new(Suit::Leaves, Rank::Ace),
                   Card::new(Suit::Leaves, Rank::Ten)],
        trump: Suit::Leaves,
        ..Default::default()
    };

    let player1_marker = game.player_on_turn();

    assert!(!game.is_closed());

    assert!(game.can_close(player1_marker).is_err());
    let result = game.close(player1_marker);

    assert_eq!(Err(ErrorKind::NotEnoughCardsInStock), result);
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

    let trump_unter = Card::new(trump, Rank::Unter);

    let expected_error = Err(ErrorKind::NoSuchCardInHand(trump_unter));
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);

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

    let expected_error = Err(ErrorKind::NotEnoughCardsInStock);
    
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);

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
    let player1_marker = game.player_on_turn();

    let close_result = game.close(player1_marker);
    assert!(close_result.is_ok());
    assert!(game.is_closed());

    let expected_error = Err(ErrorKind::DeckClosed);
    
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);

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

    assert!(game.can_exchange_trump().is_ok());

    let result = game.exchange_trump();
    assert!(result.is_ok());

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

    let expected_error = Err(ErrorKind::NoSuchCardInHand(
        Card::new(Suit::Bells, Rank::Ober)));

    assert_eq!(expected_error, game.can_call_twenty(Suit::Bells));

    let result = game.call_twenty(Suit::Bells);
    assert_eq!(expected_error, result);
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
    
    let twenty_result = game.call_twenty(twenty_suit);
    assert!(twenty_result.is_ok());

    let expected_error = Err(ErrorKind::AlreadyCalledThisTwenty(twenty_suit));
    
    assert!(game.player1.twenties.contains(&twenty_suit));

    assert_eq!(expected_error, game.can_call_twenty(twenty_suit));

    let result = game.call_twenty(twenty_suit);
    assert_eq!(expected_error, result);
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

    let expected_error = Err(ErrorKind::TwentyWithTrumpSuit);

    assert_eq!(expected_error, game.can_call_twenty(twenty_suit));

    let result = game.call_twenty(twenty_suit);
    assert_eq!(expected_error, result);

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
    assert!(game.can_call_twenty(twenty_suit).is_ok());

    let result = game.call_twenty(twenty_suit);
    assert!(result.is_ok());

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

    let expected_error = Err(ErrorKind::NoSuchCardInHand(
        Card::new(trump, Rank::Ober)));
    
    assert_eq!(expected_error, game.can_call_forty());

    let result = game.call_forty();
    assert_eq!(expected_error, result);
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
    
    let forty_result = game.call_forty();
    assert!(forty_result.is_ok());
    assert_eq!(Some(trump), game.player1.forty);

    let expected_error = Err(ErrorKind::AlreadyCalledForty);
    assert_eq!(expected_error, game.can_call_forty());

    let result = game.call_forty();
    assert_eq!(expected_error, result);
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

    assert!(game.can_call_forty().is_ok());

    let result = game.call_forty();
    assert!(result.is_ok());

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

    let player1_marker = game.player_on_turn();
    
    let first_win_declaration_result = game.declare_win(player1_marker);
    assert!(first_win_declaration_result.is_ok());
    assert!(game.is_game_over());

    let expected_error = Err(ErrorKind::GameOver);
    
    assert_eq!(expected_error, game.can_declare_win(player1_marker));

    let result = game.declare_win(player1_marker);
    assert_eq!(expected_error, result);
}

#[test]
fn declare_win_not_enough() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten)];
    let player1 = Player {wins: player1_wins, ..Default::default()};
    let mut game = Game {player1, ..Default::default()};

    let player1_marker = game.player_on_turn();
    
    let expected_error = Err(ErrorKind::ScoreTooLow(game.player1.score()));
    assert_eq!(expected_error, game.can_declare_win(player1_marker));

    let result = game.declare_win(player1_marker);
    assert_eq!(expected_error, result);
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

    let player1_marker = game.player_on_turn();
    
    assert!(game.can_declare_win(player1_marker).is_ok());

    let result = game.declare_win(player1_marker);
    assert!(result.is_ok());
}

#[test]
fn test_play_game_already_over() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten),
                            Card::new(Suit::Bells, Rank::Ace),
                            Card::new(Suit::Bells, Rank::Ten),
                            Card::new(Suit::Hearts, Rank::Ace),
                            Card::new(Suit::Hearts, Rank::Ten),
                            Card::new(Suit::Leaves, Rank::King),
                            Card::new(Suit::Leaves, Rank::Ober)];

    let player1_hand = vec![Card::new(Suit::Leaves, Rank::Unter)];
    
    let player1 = Player {hand: player1_hand, wins: player1_wins,
                          ..Default::default()};
    let mut game = Game {player1, ..Default::default()};

    let player1_marker = game.player_on_turn();

    let declare_win_result = game.declare_win(player1_marker);
    assert!(declare_win_result.is_ok());
    assert!(game.is_game_over());

    let card = game.player1.hand[0];

    let expected_error = Err(ErrorKind::GameOver);

    assert_eq!(expected_error, game.can_play_card(player1_marker, card));

    let result = game.play_card(player1_marker, card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_player_not_on_turn_first_card() {
    let mut game = Game::default();

    let card = *game.player1.hand.first().unwrap();

    let player2_marker = game.player_on_turn().other();

    let expected_error = Err(ErrorKind::NotPlayersTurn);

    assert_eq!(expected_error, game.can_play_card(player2_marker, card));
    
    let result = game.play_card(player2_marker, card);
    assert_eq!(expected_error, result);
}


#[test]
fn test_play_card_player_not_on_turn_second_card() {
    let mut game = Game::default();

    let card1 = *game.player1.hand.first().unwrap();

    let player1_marker = game.player_on_turn();

    let first_card_result = game.play_card(player1_marker, card1);
    assert!(first_card_result.is_ok());

    let card2 = *game.player1.hand.first().unwrap();
    
    let expected_error = Err(ErrorKind::NotPlayersTurn);

    assert_eq!(expected_error, game.can_play_card(player1_marker, card2));
    
    let result = game.play_card(player1_marker, card2);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_invalid_player1_card() {
    let mut game = Game::default();

    let removed_card = game.player1.hand.pop().unwrap();

    let player1_marker = game.player_on_turn();

    let expected_error = Err(ErrorKind::NoSuchCardInHand(removed_card));

    assert_eq!(expected_error,
               game.can_play_card(player1_marker, removed_card));
    
    let result = game.play_card(player1_marker, removed_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_invalid_player2_card() {
    let mut game = Game::default();

    let removed_card = game.player2.hand.pop().unwrap();
    let player1_card = *game.player1.hand.first().unwrap();

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    let first_card_result = game.play_card(player1_marker, player1_card);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::NoSuchCardInHand(removed_card));

    assert_eq!(expected_error,
               game.can_play_card(player2_marker, removed_card));
    
    let result = game.play_card(player2_marker, removed_card);
    assert_eq!(expected_error, result);
}

#[test]
pub fn test_play_card_player1_card_ok() {
    let mut game = Game::default();

    let card = *game.player1.hand.first().unwrap();

    let player1_marker = game.player_on_turn();
                             
    let result = game.play_card(player1_marker, card);
    assert!(result.is_ok());

    assert_eq!(Some(card), game.first_card_in_trick);
    assert!(!game.player1.hand.contains(&card));
    assert_eq!(player1_marker.other(), game.player_on_turn());
}

#[test]
pub fn test_play_card_player2_card_ok_not_closed() {
    test_play_card_player2_card_ok_template(false);
}

#[test]
pub fn test_play_card_player2_card_ok_closed() {
    test_play_card_player2_card_ok_template(true);
}

fn test_play_card_player2_card_ok_template(should_be_closed: bool) {
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

    let original_stock_size = stock.len();
    
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

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    if should_be_closed {
        let close_result = game.close(player1_marker);
        assert!(close_result.is_ok());
        assert!(game.is_closed());
    }

    let card1 = game.player1.hand[4];
    let card2 = game.player2.hand[4];

    let first_card_result = game.play_card(player1_marker, card1);
    assert!(first_card_result.is_ok());

    assert!(game.can_play_card(player2_marker, card2).is_ok());
    
    let result = game.play_card(player2_marker, card2);
    assert!(result.is_ok());

    assert!(!game.player1.hand.contains(&card1));
    assert!(!game.player2.hand.contains(&card2));

    let cards_less = if should_be_closed {1} else {0};
    assert_eq!(5 - cards_less, game.player1.hand.len());
    assert_eq!(5 - cards_less, game.player2.hand.len());

    let dealed_cards = if should_be_closed {0} else {2};
    assert_eq!(original_stock_size - dealed_cards, game.stock.len());
    
    assert!(game.player2.wins.contains(&card1));
    assert!(game.player2.wins.contains(&card2));
    assert_eq!(Players::Player2, game.player_on_lead());
    assert_eq!(Players::Player2, game.player_on_turn());
}

#[test]
fn test_play_card_illegal_suit() {
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

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();
    
    let closing_successful = game.close(player1_marker);

    assert!(closing_successful.is_ok());
    assert!(game.is_closed());

    let first_card_result = game.play_card(player1_marker, player1_card);
    assert!(first_card_result.is_ok());

    let expected_error
        = Err(ErrorKind::MustUseAnotherSuit(player1_card.suit()));
    assert_eq!(expected_error, game.play_card(player2_marker, player2_card));
    let result = game.play_card(player2_marker, player2_card);
    assert_eq!(expected_error, result);
}


#[test]
fn test_play_card_stock_depleted_must_be_higher() {
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

    let card1 = player1_hand[0];
    let card2 = player2_hand[0];
    let card2_takes = player2_hand[1];
    
    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();
    
    let first_card_result = game.play_card(player1_marker, card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::MustTake(card2_takes));
    assert_eq!(expected_error, game.can_play_card(player2_marker, card2));

    let result = game.play_card(player2_marker, card2);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_stock_depleted_must_use_trump() {
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

    let card1 = player1_hand[0];
    let card2 = player2_hand[0];
    
    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    let first_card_result = game.play_card(player1_marker, card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::MustUseTrump);

    assert_eq!(expected_error, game.can_play_card(player2_marker, card2));

    let result = game.play_card(player2_marker, card2);
    assert_eq!(expected_error, result);
}
