use super::*;

fn get_wins() -> Vec<Card> {
    vec![Card::new(Suit::Leaves, Rank::Ace),
         Card::new(Suit::Leaves, Rank::Ten),
         Card::new(Suit::Bells, Rank::Ace),
         Card::new(Suit::Bells, Rank::Ten),
         Card::new(Suit::Hearts, Rank::Ace),
         Card::new(Suit::Hearts, Rank::Ten),
         Card::new(Suit::Leaves, Rank::King),
         Card::new(Suit::Leaves, Rank::Ober)]
}

fn get_winning_server_game() -> Game<Stock> {
    let player1_wins = get_wins();
    let player1 = Box::new(Player::new(
        PlayerData {wins: player1_wins, ..Default::default()}));

    Game {player1, ..Default::default()}
}

#[test]
fn test_declare_win_fails_when_not_on_lead() {
    let default_game = get_winning_server_game();
    let mut game = Game {player1: default_game.player2,
                         player2: default_game.player1,
                         ..Default::default()};

    game.player1.add_to_hand(Card::new(Suit::Bells, Rank::King));
    game.player2.add_to_hand(Card::new(Suit::Hearts, Rank::King));

    check_declare_win_fails_when_not_on_lead(&mut game);
}

fn check_declare_win_fails_when_not_on_lead<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let card1 = game.get_player1().get_hand()[0];

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());
    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_declare_win());

    let result = game.declare_win();
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_win_fails_when_already_game_over() {
    let mut game = get_winning_server_game();
    
    check_declare_win_fails_when_already_game_over(&mut game);
}

fn check_declare_win_fails_when_already_game_over<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let first_win_declaration_result = game.declare_win();
    assert!(first_win_declaration_result.is_ok());
    assert!(game.is_game_over());

    let expected_error = Err(ErrorKind::GameOver);
    
    assert_eq!(expected_error, game.can_declare_win());

    let result = game.declare_win();
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_win_fails_when_score_too_low() {
    let player1_wins = vec![Card::new(Suit::Leaves, Rank::Ace),
                            Card::new(Suit::Leaves, Rank::Ten)];
    let player1 = Box::new(Player::new(
        PlayerData {wins: player1_wins, ..Default::default()}));
    let mut game = Game {player1, ..Default::default()};
    
    check_declare_win_fails_when_score_too_low(&mut game);
}

fn check_declare_win_fails_when_score_too_low<STOCK: IStock>(
    game: &mut Game<STOCK>)
{
    let expected_error = Err(ErrorKind::ScoreTooLow(game.player1.score()));
    assert_eq!(expected_error, game.can_declare_win());

    let result = game.declare_win();
    assert_eq!(expected_error, result);
}

#[test]
fn test_declare_win_ok() {
    let mut game = get_winning_server_game();
    
    check_declare_win_ok(&mut game);
}

fn check_declare_win_ok<STOCK: IStock>(game: &mut Game<STOCK>) {
    let player1_id = game.player_id_on_turn();
    assert!(game.can_declare_win().is_ok());

    let result = game.declare_win();
    assert!(result.is_ok());
    assert_eq!(Some(player1_id), game.winner());
}
