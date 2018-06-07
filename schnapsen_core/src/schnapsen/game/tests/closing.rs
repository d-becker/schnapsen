use super::*;

#[test]
fn test_trump_card_closed() {
    let cards = {
        let mut deck = generate_deck();
        deck.truncate(10);
        deck
    };
    
    let stock_data = StockData {cards, closed: true};
    let stock = Stock::new(stock_data);
    let mut game = Game {stock, ..Default::default()};

    check_no_trump_card(&mut game);
}

fn check_no_trump_card<STOCK: IStock>(game: &mut Game<STOCK>) {
    assert_eq!(None, game.trump_card());
}

#[test]
fn test_trump_card_no_card_left() {
    let cards = vec![];
    let stock_data = StockData {cards, ..Default::default()};
    let stock = Stock::new(stock_data);
    
    let mut game = Game {stock, ..Default::default()};

    check_no_trump_card(&mut game);
}

#[test]
fn test_close_ok() {
    let mut game = Game::default();
    
    check_close_ok(&mut game);
}

fn check_close_ok<STOCK: IStock>(game: &mut Game<STOCK>) {
    assert!(!game.is_closed());

    assert!(game.can_close().is_ok());
    let result = game.close();

    assert!(result.is_ok());
    assert!(game.is_closed());
}

#[test]
fn test_close_not_on_lead() {
    let mut game = Game::default();

    check_close_not_on_lead(&mut game);
}

fn check_close_not_on_lead<STOCK: IStock>(game: &mut Game<STOCK>) {
    let card1 = game.get_player1().get_hand()[0];

    let first_card_result = game.play_card(card1);
    assert!(first_card_result.is_ok());

    let expected_error = Err(ErrorKind::PlayerNotOnLead);
    assert_eq!(expected_error, game.can_close());

    let result = game.close();
    assert_eq!(expected_error, result);
}

#[test]
fn test_close_fails_when_already_closed() {
    let mut game = Game::default();

    check_close_fails_when_already_closed(&mut game);
}

fn check_close_fails_when_already_closed<STOCK: IStock>(
    game: &mut Game<STOCK>) {
    
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
fn test_close_fails_when_not_enough_cards_left() {
    let cards = vec![Card::new(Suit::Leaves, Rank::Ace),
                     Card::new(Suit::Leaves, Rank::Ten)];

    let stock = Stock::new(StockData {cards, closed: false});
    
    let mut game = Game {
        stock,
        trump: Suit::Leaves,
        ..Default::default()
    };

    check_close_fails_when_not_enough_cards_left(&mut game);
}

fn check_close_fails_when_not_enough_cards_left<STOCK: IStock>(
    game: &mut Game<STOCK>) {
    
    assert!(!game.is_closed());

    let expected_error = Err(ErrorKind::NotEnoughCardsInStock);
    assert_eq!(expected_error, game.can_close());
    let result = game.close();

    assert_eq!(expected_error, result);
    assert!(!game.is_closed());
}
