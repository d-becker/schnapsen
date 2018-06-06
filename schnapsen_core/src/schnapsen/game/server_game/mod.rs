use rand;
use rand::Rng;

use schnapsen::generate_deck;
use schnapsen::game::Game;
use schnapsen::player::{Player, PlayerData, PlayerId};
use schnapsen::stock::{Stock, StockData};

use cards::Card;

impl Default for Game<Stock> {
    fn default() -> Self {
        let deck = generate_deck();
        new_server_game(deck).unwrap()
    }
}

pub fn new_random_server_game() -> Game<Stock> {
    let mut deck = generate_deck();
    let mut rng = rand::isaac::IsaacRng::new_unseeded();
    rng.shuffle(&mut deck);

    new_server_game(deck).unwrap()
}

fn new_server_game(mut deck: Vec<Card>) -> Option<Game<Stock>> {
    // Don't allow an odd number of cards in the deck.
    if deck.len() % 2 == 1 {
        return None;
    }
    
    let deck_length = deck.len();
    let hand1 = deck.split_off(deck_length - 5);
    
    let deck_length = deck.len();
    let hand2 = deck.split_off(deck_length - 5);

    let trump = deck[0].suit();
    let stock_data = StockData {cards: deck, closed: false};
    let stock = Stock::new(stock_data);

    let player1data = PlayerData {name: "Player1".to_string(),
                                  hand: hand1, ..Default::default()};
    let player2data = PlayerData {name: "Player2".to_string(),
                                  hand: hand2, ..Default::default()};

    let game = Game {stock,
                     player1: Box::new(Player::new(player1data)),
                     player2: Box::new(Player::new(player2data)),
                     trump,
                     winner: None,
                     player_on_lead: PlayerId::Player1,
                     first_card_in_trick: None
    };

    Some(game)
}

#[cfg(test)]
mod tests;
