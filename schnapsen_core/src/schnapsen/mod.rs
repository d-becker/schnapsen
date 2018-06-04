mod game;
mod game_adapter;
mod game_data;
mod player;
mod player_game;

use cards::{Card, Suit, Rank};

use std::vec::Vec;

pub use self::game::Game;
pub use self::game_adapter::GameAdapter;
pub use self::player::Player;
pub use self::player_game::PlayerGame;
pub use self::game_data::PublicGameData;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ErrorKind {
    GameOver,
    DeckClosed,
    NotEnoughCardsInStock,
    NoSuchCardInHand(Card),
    NotTwentyCard(Card),
    AlreadyCalledThisTwenty(Suit),
    TwentyWithTrumpSuit,
    NotFortyCard(Card),
    AlreadyCalledForty,
    PlayerNotOnLead,
    NotPlayersTurn,
    MustUseAnotherSuit(Suit),
    MustTake(Card),
    MustUseTrump,
    ScoreTooLow(u32)
}

pub fn value(card: Card) -> u32 {
    match card.rank() {
        Rank::Unter => 2,
        Rank::Ober => 3,
        Rank::King => 4,
        Rank::Ten => 10,
        Rank::Ace => 11,
        _ => 0
    }
}

pub fn generate_deck() -> Vec<Card> {
    let mut vec = Vec::<Card>::new();
    const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Bells,
                              Suit::Acorns, Suit::Leaves];
    const RANKS: [Rank; 5] = [Rank::Unter, Rank::Ober, Rank::King,
                              Rank::Ten, Rank::Ace];

    for suit in SUITS.iter() {
        for rank in RANKS.iter() {
            vec.push(Card::new(*suit, *rank));
        }
    }
    
    vec
}

pub fn first_beats_second(first_card: Card,
                          second_card: Card,
                          trump: Suit)
                          -> bool {
    if first_card.suit() == second_card.suit() {
        value(first_card) > value(second_card)
    } else {
        second_card.suit() != trump
    }
}

#[cfg(test)]
mod tests;
