mod closing;
mod exchange_trump;
mod twenty;
mod forty;
mod declare_win;
mod play_card;

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
    let stock = DummyStock::new(stock_len, trump_card_rank, false);

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
