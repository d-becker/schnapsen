use super::*;

fn get_cards_ok_for_player1() -> (Vec<Card>, Vec<Card>, Vec<Card>) {
    let cards = Vec::new();
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

    (cards, hand1, hand2)
}

#[test]
fn test_play_card_twenty_not_on_lead() {
    // Note the reversed order of player1_hand and player2_hand.
    let (cards, hand2, hand1) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, hand1, hand2);

    check_play_card_twenty_not_on_lead(&mut game);
}

fn check_play_card_twenty_not_on_lead<STOCK: IStock>(game: &mut Game<STOCK>) {
    let card1 = game.get_player1().get_hand()[0];
    
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
    let (cards, mut hand1, hand2) = get_cards_ok_for_player1();

    let index = hand1.iter()
        .position(|&elem| elem == Card::new(Suit::Bells, Rank::King))
        .unwrap();
    hand1[index] = Card::new(Suit::Bells, Rank::Ten);

    let mut game = create_server_game(cards, hand1, hand2);

    check_play_card_twenty_no_such_cards(&mut game);
}

fn check_play_card_twenty_no_such_cards<STOCK: IStock>(game: &mut Game<STOCK>) {
    let twenty_card = Card::new(Suit::Bells, Rank::Ober);
    let expected_error = Err(ErrorKind::NoSuchCardInHand(
        Card::new(Suit::Bells, Rank::King)));

    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_twenty_suit_is_trump() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let twenty_card = Card::new(Suit::Bells, Rank::King);

    let trump = twenty_card.suit();

    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};
    
    check_play_card_twenty_suit_is_trump(&mut game);
}

fn check_play_card_twenty_suit_is_trump<STOCK: IStock>(game: &mut Game<STOCK>) {
    let expected_error = Err(ErrorKind::TwentyWithTrumpSuit);
    let twenty_card = Card::new(Suit::Bells, Rank::King);
    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);

    assert!(!game.player1.get_twenties().contains(&twenty_card.suit()));
}

#[test]
fn test_play_card_twenty_play_wrong_card() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, hand1, hand2);
    
    check_play_card_twenty_play_wrong_card(&mut game);
}

fn check_play_card_twenty_play_wrong_card<STOCK: IStock>(game: &mut Game<STOCK>) {
    let twenty_card = Card::new(Suit::Leaves, Rank::Ten);

    let expected_error = Err(ErrorKind::NotTwentyCard(twenty_card));
    assert_eq!(expected_error, game.can_play_card_twenty(twenty_card));

    let result = game.play_card_twenty(twenty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_twenty_ok() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, hand1, hand2);

    check_play_card_twenty_ok(&mut game);
}

fn check_play_card_twenty_ok<STOCK: IStock>(game: &mut Game<STOCK>) {
    let twenty_card = Card::new(Suit::Bells, Rank::King);
    assert!(game.can_play_card_twenty(twenty_card).is_ok());

    let result = game.play_card_twenty(twenty_card);
    assert!(result.is_ok());
    assert_eq!(Some(twenty_card), game.first_card_in_trick);

    assert!(game.player1.get_twenties().contains(&twenty_card.suit()));
}

#[test]
fn test_declare_twenty_fails_when_score_too_low() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let mut game = create_server_game(cards, hand1, hand2);
    
    check_declare_twenty_fails_when_score_too_low(&mut game);
}

fn check_declare_twenty_fails_when_score_too_low<STOCK: IStock>(game: &mut Game<STOCK>) {
    let twenty_suit = Suit::Bells;

    let score = game.get_player1().score();
    let expected_error = Err(ErrorKind::ScoreTooLow(score));
    assert_eq!(expected_error, game.can_declare_twenty_win(twenty_suit));

    let result = game.declare_twenty_win(twenty_suit);
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_twenty_ok() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let wins1 = vec![Card::new(Suit::Acorns, Rank::Ace),
                     Card::new(Suit::Acorns, Rank::Ten),
                     Card::new(Suit::Hearts, Rank::Ace),
                     Card::new(Suit::Hearts, Rank::Ten),
                     Card::new(Suit::Leaves, Rank::King)];

    let player1 = Box::new(Player::new(
        PlayerData {hand: hand1, wins: wins1, ..Default::default()}));

    let default_game = create_server_game(cards, vec![], hand2);
    let mut game = Game {player1, ..default_game};
    
    check_declare_twenty_ok(&mut game);
}

fn check_declare_twenty_ok<STOCK: IStock>(game: &mut Game<STOCK>) {
    let twenty_suit = Suit::Bells;
    
    assert!(game.can_declare_twenty_win(twenty_suit).is_ok());

    let result = game.declare_twenty_win(twenty_suit);
    assert!(result.is_ok());
}
