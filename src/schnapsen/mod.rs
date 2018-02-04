pub use cards::{Card, Suite, Rank};

use std::vec::Vec;

use rand;
use rand::Rng;

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
    const SUITES: [Suite; 4] = [Suite::Hearts, Suite::Bells, Suite::Acorns, Suite::Leaves];
    const RANKS: [Rank; 5] = [Rank::Unter, Rank::Ober, Rank::King, Rank::Ten, Rank::Ace];

    for suite in SUITES.iter() {
        for rank in RANKS.iter() {
            vec.push(Card::new(*suite, *rank));
        }
    }
    
    vec
}

pub fn first_beats_second(first_card: Card,
                          second_card: Card,
                          trumph: Suite)
                          -> bool {
    if first_card.suite() == second_card.suite() {
        value(first_card) > value(second_card)
    } else {
        second_card.suite() != trumph
    }
}

fn legal_second_card_in_endgame(hand1: &[Card], index1: usize,
                                hand2: &[Card], index2: usize,
                                trumph: Suite) -> bool {
    let card1 = hand1[index1];
    let card2 = hand2[index2];

    if card1.suite() == card2.suite() {
        if value(card1) < value(card2) {
            true
        } else {
            !hand2.iter().any(|&other_card| other_card.suite() == card1.suite()
                              && value(card1) < value(other_card))
        }
    } else {
        if hand2.iter().any(|&other_card| other_card.suite() == card1.suite()) {
            false
        } else if card2.suite() == trumph {
            true
        } else {
            !hand2.iter().any(|&other_card| other_card.suite() == trumph)
        }
    }
    
}

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
    wins: Vec<Card>
}

#[derive(Debug)]
pub struct Game {
    deck: Vec<Card>,
    trumph: Suite,
    folded: bool,

    player1: Player,
    player2: Player,
    player1_next: bool
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {name, hand: Vec::new(), wins: Vec::new()}
    }
}

impl Game {
    pub fn new() -> Game {
        let mut deck = generate_deck();
        let mut rng = rand::isaac::IsaacRng::new_unseeded();
        rng.shuffle(&mut deck);

        let deck_length = deck.len();
        let hand1 = deck.split_off(deck_length - 5);
        
        let deck_length = deck.len();
        let hand2 = deck.split_off(deck_length - 5);
        
        let trumph = deck[0].suite();

        Game {deck,
              trumph,
              folded: false,
              player1: Player {name: "Player1".to_string(),
                               hand: hand1, wins: Vec::new()},
              player2: Player {name: "Player2".to_string(),
                               hand: hand2, wins: Vec::new()},
              player1_next: true
        }
    }

    pub fn trumph_card(&self) -> Option<Card> {
        if self.folded {
            None
        } else {
            self.deck.first().map(|&card| card)
        }
    }

    pub fn is_folded(&self) -> bool {
        self.folded
    }
    
    pub fn fold(&mut self) -> bool {
        if self.deck.len() > 2 {
            self.folded = true;
        }

        self.folded
    }

    pub fn next_player_name(&self) -> &str {
        if self.player1_next {
            &self.player1.name
        } else {
            &self.player2.name
        }
    }

    pub fn next_turn_possible(&self,
                              player1_card_index: usize,
                              player2_card_index: usize) -> bool {
        let card1_opt = self.player1.hand.get(player1_card_index);
        let card2_opt = self.player2.hand.get(player2_card_index);

        match (card1_opt, card2_opt) {
            (Some(_), Some(_)) => {
                if self.folded || self.deck.is_empty() {
                    let ((first_hand, first_index), (second_hand, second_index))
                        = self.first_and_second_player(player1_card_index,
                                                       player2_card_index);
                    
                    legal_second_card_in_endgame(first_hand, first_index,
                                                 second_hand, second_index,
                                                 self.trumph)
                } else {
                    true
                }
            },
            _ => false
        }
    }

    pub fn next_turn(&mut self,
                     player1_card_index: usize,
                     player2_card_index: usize) -> bool {
        if !self.next_turn_possible(player1_card_index, player2_card_index) {
            return false;
        }

        let player1_card = self.player1.hand[player1_card_index];
        let player2_card = self.player2.hand[player2_card_index];

        let player1_wins = self.player1_wins(player1_card, player2_card);
        let (winning_player, winning_index, losing_player, losing_index);
        if player1_wins {
            winning_player = &mut self.player1;
            winning_index = player1_card_index;
            losing_player = &mut self.player2;
            losing_index = player2_card_index;
        } else {
            winning_player = &mut self.player2;
            winning_index = player2_card_index;
            losing_player = &mut self.player1;
            losing_index = player1_card_index;
        }

        let winning_card = winning_player.hand[winning_index];
        let losing_card = losing_player.hand[losing_index];

        winning_player.wins.extend_from_slice(&[winning_card, losing_card]);

        winning_player.hand.remove(winning_index);
        losing_player.hand.remove(losing_index);

        if let Some(card) = self.deck.pop() {
            winning_player.hand.push(card);
        };

        if let Some(card) = self.deck.pop() {
            losing_player.hand.push(card);
        };

        self.player1_next = player1_wins;        
        true
    }

    fn first_and_second_player(&self,
                               player1_index: usize,
                               player2_index: usize)
                               -> ((&[Card], usize), (&[Card], usize)) {
        let player1_tuple = (self.player1.hand.as_slice(), player1_index);
        let player2_tuple = (self.player2.hand.as_slice(), player2_index);
        if self.player1_next {
            (player1_tuple, player2_tuple)
        } else {
            (player2_tuple, player1_tuple)
        }
    }

    fn player1_wins(&self, player1_card: Card, player2_card: Card) -> bool {
        if self.player1_next {
            first_beats_second(player1_card, player2_card, self.trumph)
        } else {
            !first_beats_second(player2_card, player1_card, self.trumph)
        }
    }
}

#[cfg(test)]
mod tests;
