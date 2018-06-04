use cards::{Card, Suit};

use std::vec::Vec;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlayerId {
    Player1,
    Player2
}

impl PlayerId {
    pub fn other(&self) -> PlayerId {
        match *self {
            PlayerId::Player1 => PlayerId::Player2,
            PlayerId::Player2 => PlayerId::Player1
        }
    }
}

#[derive(Default, Debug)]
pub struct Player {
    pub(super) name: String,
    pub(super) hand: Vec<Card>,
    pub(super) wins: Vec<Card>,
    pub(super) twenties: Vec<Suit>,
    pub(super) forty: Option<Suit>
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {name, ..Default::default()}
    }

    pub fn score(&self) -> u32 {
        let tricks = sum_card_slice(&self.wins);
        let twenties = self.twenties.len() as u32 * 20;
        let forty = self.forty.map(|_| 40).unwrap_or(0);

        tricks + twenties + forty
    }
}

fn sum_card_slice(slice: &[Card]) -> u32 {
    slice.iter().map(|&card| super::value(card)).sum()
}

#[cfg(test)]
mod tests;
