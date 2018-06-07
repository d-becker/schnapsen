use super::*;

fn get_cards_ok_for_player1() -> (Vec<Card>, Vec<Card>, Vec<Card>) {
    let cards = vec![Card{suit: Suit::Hearts, rank: Rank::Ober},
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

    (cards, player1_hand, player2_hand)
}

#[test]
fn test_exchange_trump_not_on_lead() {
    // Note the reversed order of player1_hand and player2_hand.
    let (cards, player2_hand, player1_hand) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, player1_hand, player2_hand);

    check_exchange_trump_fails_when_not_on_lead(&mut game);
}

fn check_exchange_trump_fails_when_not_on_lead<STOCK: IStock>(
    game: &mut Game<STOCK>) {

    let card1 = game.get_player1().get_hand()[0];
    
    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);
}

#[test]
fn test_exchange_trump_fails_not_having_trump_unter() {
    let (cards, mut player1_hand, player2_hand) = get_cards_ok_for_player1();
    player1_hand[0] = Card::new(Suit::Bells, Rank::Unter);

    let mut game = create_server_game(cards, player1_hand, player2_hand);

    check_exchange_trump_fails_not_having_trump_unter(&mut game);
}

fn check_exchange_trump_fails_not_having_trump_unter<STOCK: IStock>(
    game: &mut Game<STOCK>) {

    let trump_card = game.trump_card().unwrap();
    let trump = trump_card.suit();
    
    let trump_unter = Card::new(trump, Rank::Unter);

    let expected_error = Err(ErrorKind::NoSuchCardInHand(trump_unter));
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);

    assert_eq!(trump_card, game.trump_card().unwrap());
}

#[test]
fn test_exchange_trump_fails_when_not_enough_cards_in_stock() {
    let (mut cards, player1_hand, player2_hand) = get_cards_ok_for_player1();
    cards.truncate(2);

    let mut game = create_server_game(cards, player1_hand, player2_hand);

    check_exchange_trump_fails_when_not_enough_cards_in_stock(&mut game);
}

fn check_exchange_trump_fails_when_not_enough_cards_in_stock<STOCK: IStock>(
    game: &mut Game<STOCK>) {

    let trump_card = game.trump_card().unwrap();

    let expected_error = Err(ErrorKind::NotEnoughCardsInStock);
    
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);

    assert_eq!(trump_card, game.trump_card().unwrap());
}

#[test]
fn test_exchange_trump_fails_when_closed() {
    let (cards, player1_hand, player2_hand) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, player1_hand, player2_hand);

    check_exchange_trump_fails_when_closed(&mut game);
}

fn check_exchange_trump_fails_when_closed<STOCK: IStock>(
    game: &mut Game<STOCK>) {

    let trump = game.trump_card().unwrap().suit();

    let close_result = game.close();
    assert!(close_result.is_ok());
    assert!(game.is_closed());

    let expected_error = Err(ErrorKind::DeckClosed);
    
    assert_eq!(expected_error, game.can_exchange_trump());

    let result = game.exchange_trump();
    assert_eq!(expected_error, result);

    assert_eq!(None, game.trump_card());
    assert!(game.player1.get_hand().contains(&Card::new(trump, Rank::Unter)));
}

#[test]
fn test_exchange_trump_ok() {
    let (cards, player1_hand, player2_hand) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, player1_hand, player2_hand);

    check_exchange_trump_ok(&mut game);
}

fn check_exchange_trump_ok<STOCK: IStock>(
    game: &mut Game<STOCK>) {

    let trump_card = game.trump_card().unwrap();
    let trump = trump_card.suit();

    assert!(game.can_exchange_trump().is_ok());

    let result = game.exchange_trump();
    assert!(result.is_ok());

    assert_eq!(Card::new(trump, Rank::Unter), game.trump_card().unwrap());
    assert!(game.player1.get_hand().contains(&trump_card));
}
