mod closing;
mod exchange_trump;
mod twenty;
mod forty;

use super::*;
use schnapsen::generate_deck;
use schnapsen::player::{DummyPlayer, Player, PlayerData, PlayerId};

use schnapsen::stock::{DummyStock, Stock, StockData};

fn create_server_game(stock: Vec<Card>, hand1: Vec<Card>, hand2: Vec<Card>)
                      -> Game<Stock> {
    let trump = stock.get(0).map(|card| card.suit()).unwrap_or(Suit::Hearts);
    
    let stock = Stock::new(StockData {cards: stock, closed: false});

    let player1 = Box::new(Player::new(
        PlayerData {hand: hand1, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: hand2, ..Default::default()}));

    Game {stock, trump, player1, player2, ..Default::default()}
}

fn create_client_game(trump: Suit,
                      trump_card_rank: Option<Rank>,
                      stock_len: usize,
                      player_id: PlayerId,
                      hand: Vec<Card>) -> Game<DummyStock> {

    let trump_card = trump_card_rank.map(|rank| Card::new(trump, rank));
    
    let stock = DummyStock::new(stock_len, trump_card);

    let real_player : Box<IPlayer> = Box::new(Player::new(
        PlayerData {hand, ..Default::default()}));
    let dummy_player : Box<IPlayer> = Box::new(DummyPlayer);

    let (player1, player2) = match player_id {
        PlayerId::Player1 => (real_player, dummy_player),
        PlayerId::Player2 => (dummy_player, real_player)
    };

    Game {stock, trump, player1, player2, winner: None,
          player_on_lead: PlayerId::Player1, first_card_in_trick: None}
}

#[test]
fn test_declare_win_not_on_lead() {
    let cards = Vec::new();

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
    
    let player1 = Box::new(Player::new(
        PlayerData {hand: hand1, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: hand2, wins: player2_wins, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});
    
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
    let player1 = Box::new(Player::new(
        PlayerData {wins: player1_wins, ..Default::default()}));
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
    let player1 = Box::new(Player::new(
        PlayerData {wins: player1_wins, ..Default::default()}));
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
    let player1 = Box::new(Player::new(
        PlayerData {wins: player1_wins, ..Default::default()}));
    let mut game = Game {player1, ..Default::default()};

    let player1_id = game.player_id_on_turn();
    assert!(game.can_declare_win().is_ok());

    let result = game.declare_win();
    assert!(result.is_ok());
    assert_eq!(Some(player1_id), game.winner());
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

    let trump = cards[0].suit();

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
    
    let player1 = Box::new(Player::new(
        PlayerData {hand: player1_hand, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: player2_hand, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
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
    let cards = Vec::new();

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
    
    let player1 = Box::new(Player::new(
        PlayerData {hand: player1_hand, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: player2_hand, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
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
    let cards = Vec::new();

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
    
    let player1 = Box::new(Player::new(
        PlayerData {hand: player1_hand, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: player2_hand, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
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
    
    let trump = cards[0].suit();

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

    let player1 = Box::new(Player::new(
        PlayerData {hand: player1_hand, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: player2_hand, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});

    let mut game = Game {
        stock, trump, player1, player2, ..Default::default()
    };

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

    let player1 = Box::new(Player::new(
        PlayerData {hand: hand1, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: hand2, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});

    let trump = Suit::Leaves;
    let mut game = Game {stock, trump, player1, player2, ..Default::default()};

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

    let player1 = Box::new(Player::new(
        PlayerData {hand: hand1, ..Default::default()}));
    let player2 = Box::new(Player::new(
        PlayerData {hand: hand2, ..Default::default()}));

    let stock = Stock::new(StockData {cards, closed: false});

    let trump = Suit::Leaves;
    let mut game = Game {stock, trump, player1, player2,
                         ..Default::default()};

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
