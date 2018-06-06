use super::*;
use schnapsen::stock::IStock;

#[test]
fn test_new_with_odd_number_of_cards_fails() {
    let mut deck = generate_deck();
    let card_popped = deck.pop();

    assert!(card_popped.is_some());
    assert_eq!(1, deck.len() % 2);

    let game_option = new_server_game(deck);
    assert!(game_option.is_none());
}

#[test]
fn test_trump_card_available() {
    let game = Game::default();

    assert_eq!(game.stock.trump_card(), game.trump_card());
}

#[test]
fn test_new_random_game() {
    let game = new_random_server_game();

    assert_eq!(10, game.stock.len());
    assert_eq!(5, game.player1.get_hand().len());
    assert_eq!(5, game.player2.get_hand().len());

    assert_eq!(game.stock.trump_card().unwrap().suit(), game.trump());
}
