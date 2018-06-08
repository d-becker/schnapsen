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
fn test_play_card_forty_fails_when_not_on_lead() {
    // Note the reversed order of player1_hand and player2_hand.
    let (cards, hand2, hand1) = get_cards_ok_for_player1();

    let trump = Suit::Bells;
    
    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};

    check_play_card_forty_fails_when_not_on_lead(&mut game);
}

fn check_play_card_forty_fails_when_not_on_lead<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let card1 = game.get_player1().get_hand()[0];

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    
    let forty_card = Card::new(game.trump(), Rank::King);
    assert_eq!(expected_error, game.can_play_card_forty(forty_card));

    let result = game.play_card_forty(forty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_forty_fails_when_no_such_card() {
    let (cards, mut hand1, hand2) = get_cards_ok_for_player1();

    let index = hand1.iter()
        .position(|&elem| elem == Card::new(Suit::Bells, Rank::Ober))
        .unwrap();
    hand1[index] = Card::new(Suit::Bells, Rank::Ace);

    let trump = Suit::Bells;

    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};
    
    check_play_card_forty_fails_when_no_such_card(&mut game);
}

fn check_play_card_forty_fails_when_no_such_card<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let forty_card = Card::new(game.trump(), Rank::Ober);
    
    let expected_error = Err(ErrorKind::NoSuchCardInHand(forty_card));
    
    assert_eq!(expected_error, game.can_play_card_forty(forty_card));

    let result = game.play_card_forty(forty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_forty_fails_when_wrong_card() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let trump = Suit::Bells;

    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};

    check_play_card_forty_fails_when_wrong_card(&mut game);
}

fn check_play_card_forty_fails_when_wrong_card<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let forty_card = Card::new(game.trump(), Rank::Ten);
    let expected_error = Err(ErrorKind::NotFortyCard(forty_card));
    assert_eq!(expected_error, game.can_play_card_forty(forty_card));

    let result = game.play_card_forty(forty_card);
    assert_eq!(expected_error, result);
}

#[test]
fn test_play_card_forty_ok() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let trump = Suit::Bells;
    
    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};

    check_play_card_forty_ok(&mut game);
}

fn check_play_card_forty_ok<STOCK: IStock>(game: &mut Game<STOCK>) {
    let forty_card = Card::new(game.trump(), Rank::King);
    assert!(game.can_play_card_forty(forty_card).is_ok());

    let result = game.play_card_forty(forty_card);
    assert!(result.is_ok());
    assert_eq!(Some(forty_card), game.first_card_in_trick);
    assert_eq!(Some(game.trump()), *game.player1.get_forty());
}

#[test]
fn test_declare_win_forty_fails_when_score_too_low() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let trump = Suit::Bells;
    let default_game = create_server_game(cards, hand1, hand2);
    let mut game = Game {trump, ..default_game};

    check_declare_win_forty_fails_when_score_too_low(&mut game);
}

fn check_declare_win_forty_fails_when_score_too_low<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let score = game.get_player1().score();
    let expected_error = Err(ErrorKind::ScoreTooLow(score));
    assert_eq!(expected_error, game.can_declare_forty_win());

    let result = game.declare_forty_win();
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_win_forty_ok() {
    let (cards, hand1, hand2) = get_cards_ok_for_player1();

    let wins1 = vec![Card::new(Suit::Acorns, Rank::Ace),
                     Card::new(Suit::Acorns, Rank::Ten),
                     Card::new(Suit::Hearts, Rank::Ace),
                     Card::new(Suit::Hearts, Rank::Ten),
                     Card::new(Suit::Leaves, Rank::King)];

    let player1 = Box::new(Player::new(
        PlayerData {hand: hand1, wins: wins1, ..Default::default()}));

    let trump = Suit::Bells;

    let default_game = create_server_game(cards, vec![], hand2);
    let mut game = Game {trump, player1, ..default_game};

    check_declare_win_forty_ok(&mut game);
}

fn check_declare_win_forty_ok<STOCK: IStock>(game: &mut Game<STOCK>) {
    assert!(game.can_declare_forty_win().is_ok());

    let result = game.declare_forty_win();
    assert!(result.is_ok());
}
