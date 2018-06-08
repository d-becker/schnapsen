use cards::{Card, Suit};

use std::fmt::Debug;
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

pub trait IPlayer : Debug {
    fn get_name(&self) -> &str;

    fn get_hand(&self) -> &[Card];
    fn add_to_hand(&mut self, card: Card);
    fn remove_from_hand(&mut self, card: Card);

    fn get_wins(&self) -> &[Card];
    fn add_to_wins(&mut self, card1: Card, card2: Card);

    fn get_twenties(&self) -> &[Suit];
    fn add_twenty(&mut self, suit: Suit);

    fn get_forty(&self) -> &Option<Suit>;
    fn add_forty(&mut self, suit: Suit);

    fn score(&self) -> u32;
}

#[derive(Default, Debug)]
pub struct PlayerData {
    pub name: String,
    pub hand: Vec<Card>,
    pub wins: Vec<Card>,
    pub twenties: Vec<Suit>,
    pub forty: Option<Suit>
}

#[derive(Default, Debug)]
pub struct Player {
    data: PlayerData
}

impl Player {
    pub fn new(data: PlayerData) -> Player {
        Player {data}
    }
}

impl IPlayer for Player {
    fn get_name(&self) -> &str {
        &self.data.name
    }

    fn get_hand(&self) -> &[Card] {
        &self.data.hand
    }
    
    fn add_to_hand(&mut self, card: Card) {
        self.data.hand.push(card);
    }
    
    fn remove_from_hand(&mut self, card: Card) {
        let index_option = self.data.hand.iter().position(
            |&card_in_hand| card_in_hand == card);
        index_option.map(|index| self.data.hand.remove(index));
    }

    fn get_wins(&self) -> &[Card] {
        &self.data.wins
    }
    
    fn add_to_wins(&mut self, card1: Card, card2: Card) {
        self.data.wins.push(card1);
        self.data.wins.push(card2);
    }

    fn get_twenties(&self) -> &[Suit] {
        &self.data.twenties
    }
    
    fn add_twenty(&mut self, suit: Suit) {
       self.data.twenties.push(suit);
    }

    fn get_forty(&self) -> &Option<Suit> {
        &self.data.forty
    }
    
    fn add_forty(&mut self, suit: Suit) {
        self.data.forty = Some(suit);
    }
    
    fn score(&self) -> u32 {
        let tricks = sum_card_slice(&self.data.wins);
        let twenties = self.data.twenties.len() as u32 * 20;
        let forty = self.data.forty.map(|_| 40).unwrap_or(0);

        tricks + twenties + forty
    }
}

fn sum_card_slice(slice: &[Card]) -> u32 {
    slice.iter().map(|&card| super::value(card)).sum()
}

#[derive(Debug)]
pub struct DummyPlayer;

impl IPlayer for DummyPlayer {
    fn get_name(&self) -> &str {
        &""
    }

    fn get_hand(&self) -> &[Card] {
        &[]
    }
    
    fn add_to_hand(&mut self, _card: Card) {
    }
    
    fn remove_from_hand(&mut self, _card: Card) {
    }

    fn get_wins(&self) -> &[Card] {
        &[]
    }
    
    fn add_to_wins(&mut self, _card1: Card, _card2: Card) {
    }

    fn get_twenties(&self) -> &[Suit] {
        &[]
    }
    
    fn add_twenty(&mut self, _suit: Suit) {
    }

    fn get_forty(&self) -> &Option<Suit> {
        &None
    }
    
    fn add_forty(&mut self, _suit: Suit) {
    }
    
    fn score(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests;
