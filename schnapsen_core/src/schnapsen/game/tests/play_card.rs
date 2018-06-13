use super::*;

#[test]
fn test_play_card_game_already_over() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten),
                            Card::new(Suit::Bells, Rank::Ace),
                            Card::new(Suit::Bells, Rank::Ten),
                            Card::new(Suit::Hearts, Rank::Ace),
                            Card::new(Suit::Hearts, Rank::Ten),
                            Card::new(Suit::Leaves, Rank::King),
                            Card::new(Suit::Leaves, Rank::Ober)];

    let player1_hand = vec![Card::new(Suit::Leaves, Rank::Unter)];
    
    let player1 = Box::new(Player::new(
        PlayerData {hand: player1_hand,
                    wins: player1_wins, ..Default::default()}));
    let mut game = Game {player1, ..Default::default()};

    let declare_win_result = game.declare_win();
    assert!(declare_win_result.is_ok());
    assert!(game.is_game_over());

    let card = game.player1.get_hand()[0];

    let expected_error = ErrorKind::GameOver;

    assert_eq!(Err(expected_error), game.can_play_card(card));

    let result = game.play_card(card);
    assert_eq!(Err(expected_error), result);
}

#[test]
fn test_play_card_invalid_player1_card() {
    let mut game = Game::default();

    let removed_card = *game.player1.get_hand().last().unwrap();
    game.player1.remove_from_hand(removed_card);

    let expected_error = ErrorKind::NoSuchCardInHand(removed_card);

    assert_eq!(Err(expected_error), game.can_play_card(removed_card));
    
    let result = game.play_card(removed_card);
    assert_eq!(Err(expected_error), result);
}

#[test]
fn test_play_card_invalid_player2_card() {
    let mut game = Game::default();

    let removed_card = *game.player2.get_hand().last().unwrap();
    game.player2.remove_from_hand(removed_card);
    let player1_card = *game.player1.get_hand().first().unwrap();

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

    let card = *game.player1.get_hand().first().unwrap();

    let player1_id = game.player_id_on_turn();
                             
    let result = game.play_card(card);
    assert!(result.is_ok());

    assert_eq!(Some(card), game.first_card_in_trick);
    assert!(!game.player1.get_hand().contains(&card));
    assert_eq!(player1_id.other(), game.player_id_on_turn());
}

#[test]
fn test_play_card_illegal_suit() {
    let cards = vec![Card{suit: Suit::Hearts, rank: Rank::Unter},
                     Card{suit: Suit::Hearts, rank: Rank::Ober},
                     Card{suit: Suit::Hearts, rank: Rank::King},
                     Card{suit: Suit::Hearts, rank: Rank::Ten},
                     Card{suit: Suit::Hearts, rank: Rank::Ace},
                     Card{suit: Suit::Acorns, rank: Rank::Unter}];

    let hand1 = vec![Card{suit: Suit::Acorns, rank: Rank::Ten},
                     Card{suit: Suit::Acorns, rank: Rank::Ace},
                     Card{suit: Suit::Leaves, rank: Rank::Unter},
                     Card{suit: Suit::Leaves, rank: Rank::Ober},
                     Card{suit: Suit::Leaves, rank: Rank::King}];

    let hand2 = vec![Card{suit: Suit::Acorns, rank: Rank::Ober},
                     Card{suit: Suit::Acorns, rank: Rank::King},
                     Card{suit: Suit::Leaves, rank: Rank::Ten},
                     Card{suit: Suit::Leaves, rank: Rank::Ace},
                     Card{suit: Suit::Bells, rank: Rank::Unter}];

    let mut game = create_server_game(cards, hand1, hand2);

    let card1 = game.get_player1().get_hand()[0];
    let card2 = game.get_player2().get_hand()[2];
    
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

fn get_default_server_endgame_cards() -> (Vec<Card>, Vec<Card>, Vec<Card>) {
    let cards = Vec::new();
    
    let hand1 = vec![Card{suit: Suit::Acorns, rank: Rank::Ten},
                     Card{suit: Suit::Acorns, rank: Rank::King},
                     Card{suit: Suit::Leaves, rank: Rank::Unter},
                     Card{suit: Suit::Leaves, rank: Rank::Ober},
                     Card{suit: Suit::Leaves, rank: Rank::King}];

    let hand2 = vec![Card{suit: Suit::Acorns, rank: Rank::Ober},
                     Card{suit: Suit::Acorns, rank: Rank::Ace},
                     Card{suit: Suit::Leaves, rank: Rank::Ten},
                     Card{suit: Suit::Leaves, rank: Rank::Ace},
                     Card{suit: Suit::Bells, rank: Rank::Unter}];

    (cards, hand1, hand2)
}

#[test]
fn test_play_card_stock_depleted_must_be_higher() {
    let (cards, hand1, hand2) = get_default_server_endgame_cards();
    let mut game = create_server_game(cards, hand1, hand2);

    let card1 = game.get_player1().get_hand()[0];
    let card2 = game.get_player2().get_hand()[0];
    let card2_takes = game.get_player2().get_hand()[1];
    
    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = ErrorKind::MustTake(card2_takes);
    assert_eq!(Err(expected_error), game.can_play_card(card2));

    let result = game.play_card(card2);
    assert_eq!(Err(expected_error), result);
}

#[test]
fn test_play_card_stock_depleted_must_use_trump() {
    let (cards, hand1, _) = get_default_server_endgame_cards();

    let hand2 = vec![Card{suit: Suit::Bells, rank: Rank::Ober},
                     Card{suit: Suit::Bells, rank: Rank::Ace},
                     Card{suit: Suit::Leaves, rank: Rank::Ten},
                     Card{suit: Suit::Leaves, rank: Rank::Ace},
                     Card{suit: Suit::Hearts, rank: Rank::Unter}];

    let mut game = create_server_game(cards, hand1, hand2);

    let card1 = game.get_player1().get_hand()[0];
    let card2 = game.get_player2().get_hand()[0];

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
    let cards = vec![
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

    let first_card_to_deal = *cards.last().unwrap();
    let second_card_to_deal = cards[cards.len() - 2];

    let original_stock_size = cards.len();

    let hand1 = vec![
        Card{suit: Suit::Bells, rank: Rank::Unter},
        Card{suit: Suit::Bells, rank: Rank::Ober},
        Card{suit: Suit::Bells, rank: Rank::King},
        Card{suit: Suit::Leaves, rank: Rank::Ace},
        Card{suit: Suit::Bells, rank: Rank::Ten},
    ];

    let hand2 = vec![
        Card{suit: Suit::Leaves, rank: Rank::Unter},
        Card{suit: Suit::Leaves, rank: Rank::Ober},
        Card{suit: Suit::Leaves, rank: Rank::King},
        Card{suit: Suit::Leaves, rank: Rank::Ten},
        Card{suit: Suit::Bells, rank: Rank::Ace},
    ];

    let mut game = create_server_game(cards, hand1, hand2);

    let player1_id = game.player_id_on_turn();
    let player2_id = player1_id.other();

    if should_be_closed {
        let close_result = game.close();
        assert!(close_result.is_ok());
        assert!(game.is_closed());
    }

    let card1 = game.player1.get_hand()[4];
    let card2 = game.player2.get_hand()[4];

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

    assert!(!game.player1.get_hand().contains(&card1));
    assert!(!game.player2.get_hand().contains(&card2));

    let cards_less = if should_be_closed {1} else {0};
    assert_eq!(5 - cards_less, game.player1.get_hand().len());
    assert_eq!(5 - cards_less, game.player2.get_hand().len());

    let dealed_cards = if should_be_closed {0} else {2};
    assert_eq!(original_stock_size - dealed_cards, game.stock.len());
    
    assert!(game.player2.get_wins().contains(&card1));
    assert!(game.player2.get_wins().contains(&card2));
    assert_eq!(player2_id, game.player_id_on_lead());
    assert_eq!(player2_id, game.player_id_on_turn());
}

#[test]
fn test_play_not_last_cards_stock_depleted_no_game_over() {
    let cards = Vec::new();

    let card1 = Card::new(Suit::Leaves, Rank::Ten);
    let card2 = Card::new(Suit::Leaves, Rank::Ace);
    
    let hand1 = vec![card1, Card::new(Suit::Leaves, Rank::Ober)];
    let hand2 = vec![card2, Card::new(Suit::Leaves, Rank::King)];

    let trump = Suit::Leaves;
    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};

    let player1_id = game.player_id_on_turn();
    let player2_id = player1_id.other();

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    assert_eq!(player2_id, game.player_id_on_turn());

    let second_card_result = game.play_card(card2);
    assert!(second_card_result.is_ok());

    assert!(!game.is_game_over());
    assert_eq!(None, game.winner());
}

#[test]
fn test_play_last_cards_game_over() {
    let cards = Vec::new();

    let card1 = Card::new(Suit::Leaves, Rank::Ten);
    let card2 = Card::new(Suit::Leaves, Rank::Ace);
    
    let hand1 = vec![card1];
    let hand2 = vec![card2];

    let trump = Suit::Leaves;

    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};
                         
    let player1_id = game.player_id_on_turn();
    let player2_id = player1_id.other();

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    assert_eq!(player2_id, game.player_id_on_turn());

    let second_card_result = game.play_card(card2);
    assert!(second_card_result.is_ok());

    assert!(game.is_game_over());
    assert_eq!(Some(player2_id), game.winner());
}
