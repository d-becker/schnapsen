pub use cards::{Card, Suit, Rank};

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

fn legal_second_card_in_endgame(_hand1: &[Card], card1: Card,
                                hand2: &[Card], card2: Card,
                                trump: Suit) -> bool {
    if card1.suit() == card2.suit() {
        if value(card1) < value(card2) {
            true
        } else {
            !hand2.iter().any(|&other_card| other_card.suit() == card1.suit()
                              && value(card1) < value(other_card))
        }
    } else {
        if hand2.iter().any(|&other_card| other_card.suit() == card1.suit()) {
            false
        } else if card2.suit() == trump {
            true
        } else {
            !hand2.iter().any(|&other_card| other_card.suit() == trump)
        }
    }
    
}

fn sum_card_slice(slice: &[Card]) -> u32 {
    slice.iter().map(|&card| value(card)).sum()
}

#[derive(Default, Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
    wins: Vec<Card>,
    twenties: Vec<Suit>,
    forty: Option<Suit>
}

#[derive(Debug)]
pub struct Game {
    deck: Vec<Card>,
    trump: Suit,
    closed: bool,

    player1: Player,
    player2: Player,
    player1_next: bool
}

impl Default for Game {
    fn default() -> Self {
        let deck = generate_deck();
        Game::new_(deck)
    }
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

impl Game {
    fn new_(mut deck: Vec<Card>) -> Game {
        let deck_length = deck.len();
        let hand1 = deck.split_off(deck_length - 5);
        
        let deck_length = deck.len();
        let hand2 = deck.split_off(deck_length - 5);
        
        let trump = deck[0].suit();

        Game {deck,
              trump,
              closed: false,
              player1: Player {name: "Player1".to_string(),
                               hand: hand1, ..Default::default()},
              player2: Player {name: "Player2".to_string(),
                               hand: hand2, ..Default::default()},
              player1_next: true
        }
    }
    
    pub fn new_random() -> Game {
        let mut deck = generate_deck();
        let mut rng = rand::isaac::IsaacRng::new_unseeded();
        rng.shuffle(&mut deck);

        Game::new_(deck)
    }

    pub fn trump_card(&self) -> Option<Card> {
        if self.closed {
            None
        } else {
            self.deck.first().map(|&card| card)
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn can_close(&self) -> bool {
        !self.is_closed() && self.deck.len() > 2
    }
    
    pub fn close(&mut self) -> bool {
        let can_close = self.can_close();

        if can_close {
            self.closed = true;
        }
        
        can_close
    }

    pub fn can_call_twenty(&self, suit: Suit) -> bool {
        let current_player = if self.player1_next {
            &self.player1
        } else {
            &self.player2
        };

        current_player.hand.contains(&Card::new(suit, Rank::Ober))
            && current_player.hand.contains(&Card::new(suit, Rank::King))
            && !current_player.twenties.contains(&suit)
            && suit != self.trump
    }

    pub fn call_twenty(&mut self, suit: Suit) -> bool {
        let can_call_twenty = self.can_call_twenty(suit);

        if can_call_twenty {
            let current_player = if self.player1_next {
                &mut self.player1
            } else {
                &mut self.player2
            };

            current_player.twenties.push(suit);
        }
        
        can_call_twenty
    }

    pub fn can_call_forty(&self) -> bool {
        let current_player = if self.player1_next {
            &self.player1
        } else {
            &self.player2
        };
        
        current_player.hand.contains(&Card::new(self.trump, Rank::Ober))
            && current_player.hand.contains(&Card::new(self.trump, Rank::King))
            && current_player.forty.is_none()
    }

    pub fn call_forty(&mut self) -> bool {
        let can_call_forty = self.can_call_forty();
        if can_call_forty {
            let current_player = if self.player1_next {
                &mut self.player1
            } else {
                &mut self.player2
            };

            current_player.forty = Some(self.trump);
        }

        can_call_forty
    }
    
    pub fn next_player_name(&self) -> &str {
        if self.player1_next {
            &self.player1.name
        } else {
            &self.player2.name
        }
    }

    pub fn next_turn_possible(&self,
                              player1_card: Card,
                              player2_card: Card) -> bool {
        if !self.player1.hand.contains(&player1_card)
            || !self.player2.hand.contains(&player2_card) {
            return false;
        }

        let is_endgame = self.closed || self.deck.is_empty();
        if !is_endgame {
            return true;
        }
        
        let ((first_hand, first_card), (second_hand, second_card))
            = self.first_and_second_player(player1_card,
                                           player2_card);
        
        legal_second_card_in_endgame(first_hand, first_card,
                                     second_hand, second_card,
                                     self.trump)
    }

    pub fn next_turn(&mut self,
                     player1_card: Card,
                     player2_card: Card) -> bool {
        if !self.next_turn_possible(player1_card, player2_card) {
            return false;
        }

        let player1_wins = self.player1_wins(player1_card, player2_card);
        let (winning_player, winning_card, losing_player, losing_card);
        if player1_wins {
            winning_player = &mut self.player1;
            winning_card = player1_card;
            losing_player = &mut self.player2;
            losing_card = player2_card;
        } else {
            winning_player = &mut self.player2;
            winning_card = player2_card;
            losing_player = &mut self.player1;
            losing_card = player1_card;
        }

        winning_player.wins.extend_from_slice(&[winning_card, losing_card]);

        let winning_index = winning_player.hand.iter()
            .position(|&card| card == winning_card).unwrap();
        winning_player.hand.remove(winning_index);

        let losing_index = losing_player.hand.iter()
            .position(|&card| card == losing_card).unwrap();
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
                               player1_card: Card,
                               player2_card: Card)
                               -> ((&[Card], Card), (&[Card], Card)) {
        let player1_tuple = (self.player1.hand.as_slice(), player1_card);
        let player2_tuple = (self.player2.hand.as_slice(), player2_card);
        if self.player1_next {
            (player1_tuple, player2_tuple)
        } else {
            (player2_tuple, player1_tuple)
        }
    }

    fn player1_wins(&self, player1_card: Card, player2_card: Card) -> bool {
        if self.player1_next {
            first_beats_second(player1_card, player2_card, self.trump)
        } else {
            !first_beats_second(player2_card, player1_card, self.trump)
        }
    }
}

#[cfg(test)]
mod tests;
