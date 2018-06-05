use super::*;

#[test]
fn test_new_with_odd_number_of_cards_fails() {
    let mut deck = generate_deck();
    let card_popped = deck.pop();

    assert!(card_popped.is_some());
    assert_eq!(1, deck.len() % 2);

    let game_option = Game::new_(deck);
    assert!(game_option.is_none());
}

#[test]
fn test_new_random_game() {
    let game = Game::new_random();

    assert_eq!(10, game.stock.len());
    assert_eq!(5, game.player1.hand.len());
    assert_eq!(5, game.player2.hand.len());

    assert_eq!(game.stock[0].suit(), game.trump());
}

#[test]
fn test_trump_card_available() {
    let game = Game::default();

    assert_eq!(Some(game.stock[0]), game.trump_card());
}

#[test]
fn test_trump_card_closed() {
    let public_data = PublicGameData {closed: true, ..Default::default()};
    let game = Game {public_data, ..Default::default()};

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

    assert!(game.can_close().is_ok());
    let result = game.close();

    assert!(result.is_ok());
    assert!(game.is_closed());
}

#[test]
fn test_close_not_on_lead() {
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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let card1 = game.player1.hand[0];

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_close());

    let result = game.close();
    assert_eq!(expected_error, result);
}

#[test]
fn test_close_already_closed() {
    let mut game = Game::default();

    assert!(!game.is_closed());

    let close_result = game.close();
    assert!(close_result.is_ok());
    assert!(game.is_closed());

    let expected_error = Err(ErrorKind::DeckClosed);
    assert_eq!(expected_error, game.can_close());
    
    let result = game.close();
    assert_eq!(expected_error, result);
}

#[test]
fn test_close_not_enough_cards_left() {
    let public_data = PublicGameData {trump: Suit::Leaves,
                                      ..Default::default()};
    let mut game = Game {
        stock: vec![Card::new(Suit::Leaves, Rank::Ace),
                   Card::new(Suit::Leaves, Rank::Ten)],
        public_data,
        ..Default::default()
    };

    assert!(!game.is_closed());

    let expected_error = Err(ErrorKind::NotEnoughCardsInStock);
    assert_eq!(expected_error, game.can_close());
    let result = game.close();

    assert_eq!(expected_error, result);
    assert!(!game.is_closed());
}

#[test]
fn test_exchange_trump_not_on_lead() {
    let stock = vec![Card{suit: Suit::Hearts, rank: Rank::Ober},
                      Card{suit: Suit::Hearts, rank: Rank::King},
                      Card{suit: Suit::Hearts, rank: Rank::Ten},
                      Card{suit: Suit::Hearts, rank: Rank::Ace}];
        
    let player1_hand = vec![Card{suit: Suit::Acorns, rank: Rank::Unter},
                            Card{suit: Suit::Acorns, rank: Rank::Ober},
                            Card{suit: Suit::Acorns, rank: Rank::King},
                            Card{suit: Suit::Acorns, rank: Rank::Ten},
                            Card{suit: Suit::Acorns, rank: Rank::Ace}];

    let player2_hand = vec![Card{suit: Suit::Hearts, rank: Rank::Unter},
                            Card{suit: Suit::Bells, rank: Rank::Ober},
                            Card{suit: Suit::Bells, rank: Rank::King},
                            Card{suit: Suit::Bells, rank: Rank::Ten},
                            Card{suit: Suit::Bells, rank: Rank::Ace}];

    let card1 = player1_hand[0];
    
    let trump_card = stock[0];
    let trump = trump_card.suit();

    let player1 = Player {hand: player1_hand, ..Default::default()};
    let player2 = Player {hand: player2_hand, ..Default::default()};

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2, ..Default::default()};

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);
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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let close_result = game.close();
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
fn test_exchange_trump_ok() {
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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    assert!(game.can_exchange_trump().is_ok());

    let result = game.exchange_trump();
    assert!(result.is_ok());

    assert_eq!(Card::new(trump, Rank::Unter), game.trump_card().unwrap());
    assert!(game.player1.hand.contains(&trump_card));
}

#[test]
fn test_play_card_twenty_not_on_lead() {
    let stock = Vec::new();

    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let card1 = hand1[0];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Hearts;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    
    let twenty_card = Card::new(Suit::Bells, Rank::King);
    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_twenty_no_such_cards() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Hearts;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let twenty_card = Card::new(Suit::Bells, Rank::Ober);
    let expected_error = Err(ErrorKind::NoSuchCardInHand(
        Card::new(Suit::Bells, Rank::King)));

    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_twenty_suit_is_trump() {
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

    let twenty_card = Card::new(Suit::Bells, Rank::King);

    let public_data = PublicGameData {trump: twenty_card.suit(),
                                      ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let expected_error = Err(ErrorKind::TwentyWithTrumpSuit);

    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);

    assert!(!game.player1.twenties.contains(&twenty_card.suit()));
}

#[test]
fn test_play_card_twenty_play_wrong_card() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Ten),
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

    let trump = Suit::Hearts;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let twenty_card = Card::new(Suit::Bells, Rank::Ten);

    let expected_error = Err(ErrorKind::NotTwentyCard(twenty_card));
    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_twenty_ok() {
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

    let trump = Suit::Hearts;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let twenty_card = Card::new(Suit::Bells, Rank::King);
    assert!(game.can_play_card_twenty(twenty_card).is_ok());

    let result = game.play_card_twenty(twenty_card);
    assert!(result.is_ok());
    assert_eq!(Some(twenty_card), game.public_data.first_card_in_trick);

    assert!(game.player1.twenties.contains(&twenty_card.suit()));
}

#[test]
fn test_declare_twenty_score_too_low() {
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

    let wins1 = vec![Card::new(Suit::Acorns, Rank::Ace)];

    let player1 = Player {hand: hand1, wins: wins1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Hearts;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let twenty_suit = Suit::Bells;

    let score = game.get_player1().score();
    let expected_error = Err(ErrorKind::ScoreTooLow(score));
    assert_eq!(expected_error, game.can_declare_twenty_win(twenty_suit));

    let result = game.declare_twenty_win(twenty_suit);
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_twenty_ok() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ober)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Unter)];

    let wins1 = vec![Card::new(Suit::Acorns, Rank::Ace),
                     Card::new(Suit::Acorns, Rank::Ten),
                     Card::new(Suit::Hearts, Rank::Ace),
                     Card::new(Suit::Hearts, Rank::Ten),
                     Card::new(Suit::Leaves, Rank::King)];

    let player1 = Player {hand: hand1, wins: wins1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Hearts;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let twenty_suit = Suit::Bells;
    
    assert!(game.can_declare_twenty_win(twenty_suit).is_ok());

    let result = game.declare_twenty_win(twenty_suit);
    assert!(result.is_ok());
}

#[test]
fn test_play_card_forty_not_on_lead() {
    let stock = Vec::new();

    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Ten)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ace)];

    let card1 = hand1[0];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Bells;
    let forty_card = Card::new(trump, Rank::King);
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    
    assert_eq!(expected_error, game.can_play_card_forty(forty_card));

    let result = game.play_card_forty(forty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_forty_no_such_cards() {
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
    let forty_card = Card::new(trump, Rank::Ober);
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    let expected_error = Err(ErrorKind::NoSuchCardInHand(
        Card::new(trump, Rank::Ober)));
    
    assert_eq!(expected_error, game.can_play_card_forty(forty_card));

    let result = game.play_card_forty(forty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_forty_wrong_card() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Ten),
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
    let forty_card = Card::new(trump, Rank::Ten);
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let expected_error = Err(ErrorKind::NotFortyCard(forty_card));
    assert_eq!(expected_error, game.can_play_card_forty(forty_card));

    let result = game.play_card_forty(forty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_forty_too_low() {
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
    
    let wins1 = vec![Card::new(Suit::Acorns, Rank::Ace)];

    let player1 = Player {hand: hand1, wins: wins1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Bells;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let score = game.get_player1().score();
    let expected_error = Err(ErrorKind::ScoreTooLow(score));
    assert_eq!(expected_error, game.can_declare_forty_win());

    let result = game.declare_forty_win();
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_forty_ok() {
    let stock = Vec::new();
    let hand1 = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten),
                     Card::new(Suit::Bells, Rank::Ober),
                     Card::new(Suit::Bells, Rank::King),
                     Card::new(Suit::Hearts, Rank::Ober)];

    let hand2 = vec![Card::new(Suit::Leaves, Rank::Ober),
                     Card::new(Suit::Leaves, Rank::Unter),
                     Card::new(Suit::Bells, Rank::Ace),
                     Card::new(Suit::Bells, Rank::Unter),
                     Card::new(Suit::Hearts, Rank::Unter)];

    let wins1 = vec![Card::new(Suit::Acorns, Rank::Ace),
                     Card::new(Suit::Acorns, Rank::Ten),
                     Card::new(Suit::Hearts, Rank::Ace),
                     Card::new(Suit::Hearts, Rank::Ten),
                     Card::new(Suit::Leaves, Rank::King)];

    let player1 = Player {hand: hand1, wins: wins1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Bells;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};
    
    assert!(game.can_declare_forty_win().is_ok());

    let result = game.declare_forty_win();
    assert!(result.is_ok());
}

#[test]
fn test_play_card_forty_ok() {
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
    let forty_card = Card::new(trump, Rank::King);
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    assert!(game.can_play_card_forty(forty_card).is_ok());

    let result = game.play_card_forty(forty_card);
    assert!(result.is_ok());
    assert_eq!(Some(forty_card), game.public_data.first_card_in_trick);
    assert_eq!(Some(trump), game.player1.forty);
}

#[test]
fn test_declare_win_not_on_lead() {
    let stock = Vec::new();

    let hand1 = vec![Card::new(Suit::Bells, Rank::King)];
    let hand2 = vec![Card::new(Suit::Hearts, Rank::King)];

    let card1 = hand1[0];
    
    let player2_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten),
                            Card::new(Suit::Bells, Rank::Ace),
                            Card::new(Suit::Bells, Rank::Ten),
                            Card::new(Suit::Hearts, Rank::Ace),
                            Card::new(Suit::Hearts, Rank::Ten),
                            Card::new(Suit::Leaves, Rank::King),
                            Card::new(Suit::Leaves, Rank::Ober)];
    
    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, wins: player2_wins,
                          ..Default::default()};
    let mut game = Game {stock, player1, player2, ..Default::default()};

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());
    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_declare_win());

    let result = game.declare_win();
    assert_eq!(expected_error, result);
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
    
    let first_win_declaration_result = game.declare_win();
    assert!(first_win_declaration_result.is_ok());
    assert!(game.is_game_over());

    let expected_error = Err(ErrorKind::GameOver);
    
    assert_eq!(expected_error, game.can_declare_win());

    let result = game.declare_win();
    assert_eq!(expected_error, result);
}

#[test]
fn declare_win_not_enough() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten)];
    let player1 = Player {wins: player1_wins, ..Default::default()};
    let mut game = Game {player1, ..Default::default()};
    
    let expected_error = Err(ErrorKind::ScoreTooLow(game.player1.score()));
    assert_eq!(expected_error, game.can_declare_win());

    let result = game.declare_win();
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
    assert!(game.can_declare_win().is_ok());

    let result = game.declare_win();
    assert!(result.is_ok());
    assert_eq!(Some(player1_marker), game.winner());
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

    let declare_win_result = game.declare_win();
    assert!(declare_win_result.is_ok());
    assert!(game.is_game_over());

    let card = game.player1.hand[0];

    let expected_error = ErrorKind::GameOver;

    assert_eq!(Err(expected_error), game.can_play_card(card));

    let result = game.play_card(card);
    assert_eq!(Err(expected_error), result);
}

#[test]
fn test_play_card_invalid_player1_card() {
    let mut game = Game::default();

    let removed_card = game.player1.hand.pop().unwrap();

    let expected_error = ErrorKind::NoSuchCardInHand(removed_card);

    assert_eq!(Err(expected_error), game.can_play_card(removed_card));
    
    let result = game.play_card(removed_card);
    assert_eq!(Err(expected_error), result);
}

#[test]
fn test_play_card_invalid_player2_card() {
    let mut game = Game::default();

    let removed_card = game.player2.hand.pop().unwrap();
    let player1_card = *game.player1.hand.first().unwrap();

    let first_card_result = game.play_card(player1_card);
    assert!(first_card_result.is_ok());

    let expected_error = ErrorKind::NoSuchCardInHand(removed_card);

    assert_eq!(Err(expected_error), game.can_play_card(removed_card));
    
    let result = game.play_card(removed_card);
    assert_eq!(Err(expected_error), result);
}

#[test]
pub fn test_play_card_player1_card_ok() {
    let mut game = Game::default();

    let card = *game.player1.hand.first().unwrap();

    let player1_marker = game.player_on_turn();
                             
    let result = game.play_card(card);
    assert!(result.is_ok());

    assert_eq!(Some(card), game.public_data.first_card_in_trick);
    assert!(!game.player1.hand.contains(&card));
    assert_eq!(player1_marker.other(), game.player_on_turn());
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

    let card1 = player1_hand[0];
    let card2 = player2_hand[2];
    
    let player1 = Player {name: "Player1".to_string(), hand: player1_hand,
                          ..Default::default()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand,
                          ..Default::default()};

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {
        stock, public_data, player1, player2, ..Default::default()
    };
    
    let closing_successful = game.close();

    assert!(closing_successful.is_ok());
    assert!(game.is_closed());

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error
        = Err(ErrorKind::MustUseAnotherSuit(card1.suit()));
    assert_eq!(expected_error, game.play_card(card2));
    let result = game.play_card(card2);
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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {
        stock, public_data, player1, player2, ..Default::default()
    };
    
    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = ErrorKind::MustTake(card2_takes);
    assert_eq!(Err(expected_error), game.can_play_card(card2));

    let result = game.play_card(card2);
    assert_eq!(Err(expected_error), result);
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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {
        stock, public_data, player1, player2, ..Default::default()
    };

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = ErrorKind::MustUseTrump;

    assert_eq!(Err(expected_error), game.can_play_card(card2));

    let result = game.play_card(card2);
    assert_eq!(Err(expected_error), result);
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

    let first_card_to_deal = *stock.last().unwrap();
    let second_card_to_deal = stock[stock.len() - 2];

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

    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {
        stock, public_data, player1, player2, ..Default::default()
    };

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    if should_be_closed {
        let close_result = game.close();
        assert!(close_result.is_ok());
        assert!(game.is_closed());
    }

    let card1 = game.player1.hand[4];
    let card2 = game.player2.hand[4];

    let first_card_result = game.play_card(card1);
    assert_eq!(Ok(None), first_card_result);

    assert!(game.can_play_card(card2).is_ok());

    // Player2 has won the trick, so the first card is dealt to them.
    let expected_cards = if should_be_closed {
        None
    } else {
        Some((second_card_to_deal, first_card_to_deal))
    };
    
    let result = game.play_card(card2);
    assert_eq!(Ok(expected_cards), result);

    assert!(!game.player1.hand.contains(&card1));
    assert!(!game.player2.hand.contains(&card2));

    let cards_less = if should_be_closed {1} else {0};
    assert_eq!(5 - cards_less, game.player1.hand.len());
    assert_eq!(5 - cards_less, game.player2.hand.len());

    let dealed_cards = if should_be_closed {0} else {2};
    assert_eq!(original_stock_size - dealed_cards, game.stock.len());
    
    assert!(game.player2.wins.contains(&card1));
    assert!(game.player2.wins.contains(&card2));
    assert_eq!(player2_marker, game.player_on_lead());
    assert_eq!(player2_marker, game.player_on_turn());
}

#[test]
fn test_play_not_last_cards_stock_depleted_no_game_over() {
    let stock = Vec::new();

    let card1 = Card::new(Suit::Leaves, Rank::Ten);
    let card2 = Card::new(Suit::Leaves, Rank::Ace);
    
    let hand1 = vec![card1, Card::new(Suit::Leaves, Rank::Ober)];
    let hand2 = vec![card2, Card::new(Suit::Leaves, Rank::King)];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Leaves;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    assert_eq!(player2_marker, game.player_on_turn());

    let second_card_result = game.play_card(card2);
    assert!(second_card_result.is_ok());

    assert!(!game.is_game_over());
    assert_eq!(None, game.winner());
}

#[test]
fn test_play_last_cards_game_over() {
    let stock = Vec::new();

    let card1 = Card::new(Suit::Leaves, Rank::Ten);
    let card2 = Card::new(Suit::Leaves, Rank::Ace);
    
    let hand1 = vec![card1];
    let hand2 = vec![card2];

    let player1 = Player {hand: hand1, ..Default::default()};
    let player2 = Player {hand: hand2, ..Default::default()};

    let trump = Suit::Leaves;
    let public_data = PublicGameData {trump, ..Default::default()};
    let mut game = Game {stock, public_data, player1, player2,
                         ..Default::default()};

    let player1_marker = game.player_on_turn();
    let player2_marker = player1_marker.other();

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    assert_eq!(player2_marker, game.player_on_turn());

    let second_card_result = game.play_card(card2);
    assert!(second_card_result.is_ok());

    assert!(game.is_game_over());
    assert_eq!(Some(player2_marker), game.winner());
}
