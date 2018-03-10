use rand;
use rand::Rng;

use cards::{Card, Suit, Rank};
use schnapsen::{ErrorKind, Players, player::Player};
use schnapsen::{generate_deck, first_beats_second};

mod game_data;
mod player_game;

pub use self::game_data::PublicGameData;
pub use self::player_game::PlayerGameData;

#[derive(Debug)]
pub struct Game {
    stock: Vec<Card>,
    player1: Player,
    player2: Player,
    
    public_data: PublicGameData
}

impl Default for Game {
    fn default() -> Self {
        let deck = generate_deck();
        Game::new_(deck)
    }
}

impl Game {
    fn new_(mut deck: Vec<Card>) -> Game {
        let deck_length = deck.len();
        let hand1 = deck.split_off(deck_length - 5);
        
        let deck_length = deck.len();
        let hand2 = deck.split_off(deck_length - 5);

        let stock = deck;
        let trump = stock[0].suit();

        let public_data = PublicGameData {trump, closed: false, winner: None,
                                          player_on_lead: Players::Player1,
                                          first_card_in_trick: None};
        Game {stock,
              player1: Player {name: "Player1".to_string(),
                               hand: hand1, ..Default::default()},
              player2: Player {name: "Player2".to_string(),
                               hand: hand2, ..Default::default()},
              public_data
        }
    }
    
    pub fn new_random() -> Game {
        let mut deck = generate_deck();
        let mut rng = rand::isaac::IsaacRng::new_unseeded();
        rng.shuffle(&mut deck);

        Game::new_(deck)
    }

    pub fn get_player1(&self) -> &Player {
        &self.player1
    }

    pub fn get_player2(&self) -> &Player {
        &self.player2
    }

    pub fn get_player(&self, player: Players) -> &Player {
        match player {
            Players::Player1 => self.get_player1(),
            Players::Player2 => self.get_player2()
        }
    }

    pub fn player_on_lead(&self) -> Players {
        self.public_data.player_on_lead
    }
    
    pub fn player_on_turn(&self) -> Players {
        self.public_data.player_on_turn()
    }

    pub fn trump(&self) -> Suit {
        self.public_data.trump
    }
    
    pub fn trump_card(&self) -> Option<Card> {
        if self.is_closed() {
            None
        } else {
            self.stock.first().map(|&card| card)
        }
    }

    pub fn is_closed(&self) -> bool {
        self.public_data.closed
    }

    pub fn can_close(&self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_close()
    }
    
    pub fn close(&mut self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let mut data_as_player_mut
            = self.get_data_as_player_mut(player_on_turn);
        data_as_player_mut.close() 
    }

    pub fn is_game_over(&self) -> bool {
        self.public_data.is_game_over()
    }

    pub fn winner(&self) -> Option<Players> {
        self.public_data.winner
    }

    pub fn can_exchange_trump(&self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_exchange_trump()
    }

    pub fn exchange_trump(&mut self) -> Result<(), ErrorKind> {
        let res = {
            let player_on_turn = self.player_on_turn();
            let mut data_as_player_mut
                = self.get_data_as_player_mut(player_on_turn);

            data_as_player_mut.exchange_trump()
        };

        if res.is_ok() {
            let trump = self.public_data.trump;
            self.stock[0] = Card::new(trump, Rank::Unter);
        }

        res
    }

    pub fn can_call_twenty(&self, suit: Suit) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_call_twenty(suit)
    }

    pub fn call_twenty(&mut self, suit: Suit) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let mut data_as_player_mut
            = self.get_data_as_player_mut(player_on_turn);
        data_as_player_mut.call_twenty(suit)
    }

    pub fn can_call_forty(&self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_call_forty()
    }

    pub fn call_forty(&mut self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let mut data_as_player_mut
            = self.get_data_as_player_mut(player_on_turn);
        data_as_player_mut.call_forty()
    }

    pub fn can_declare_win(&self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_declare_win()

    }
    
    pub fn declare_win(&mut self) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let mut data_as_player_mut
            = self.get_data_as_player_mut(player_on_turn);
        data_as_player_mut.declare_win()
    }

    pub fn can_play_card(&self, card: Card) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_play_card(card)
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card(card)?;

        let player_on_turn = self.player_on_turn();

        self.remove_card_from_hand(player_on_turn, card);

        match self.public_data.first_card_in_trick {
            // We are playing the first card in the trick.
            None => self.public_data.first_card_in_trick = Some(card),

            // We are playing the second card in the trick.
            Some(card_on_lead) => {
                let player_on_lead_wins
                    = first_beats_second(card_on_lead, card,
                                         self.public_data.trump);

                let winning_player_marker = if player_on_lead_wins
                {
                    self.player_on_lead()
                } else {
                    player_on_turn
                };

                self.add_cards_to_wins(winning_player_marker,
                                       &[card_on_lead, card]);

                self.deal_if_not_closed_or_empty(winning_player_marker);
                self.public_data.player_on_lead = winning_player_marker;
                self.public_data.first_card_in_trick = None;

                // Actually the players should always have
                // the same number of cards in their hands.
                if self.player1.hand.is_empty()
                    || self.player2.hand.is_empty() {
                    self.public_data.winner = Some(winning_player_marker);
                }
            }
        }
        
        Ok(())
    }

    fn get_player_mut(&mut self, player: Players) -> &mut Player {
        match player {
            Players::Player1 => &mut self.player1,
            Players::Player2 => &mut self.player2
        }
    }

    fn get_data_as_player(&self, player_id: Players)
                          -> PlayerGameData<&Player, &PublicGameData> {
        let player = self.get_player(player_id);
        let stock_size = self.stock.len() as u32;
        let trump_card_rank = self.trump_card().map(|card| card.rank());
        let public_data = &self.public_data;
        PlayerGameData {player_id, player, stock_size,
                        trump_card_rank, public_data}
    }

    fn get_data_as_player_mut(&mut self, player_id: Players)
                              -> PlayerGameData<&mut Player, &mut PublicGameData>
    {
        let stock_size = self.stock.len() as u32;
        let trump_card_rank = self.trump_card().map(|card| card.rank());
        let public_data = &mut self.public_data;
        let player = match player_id {
            Players::Player1 => &mut self.player1,
            Players::Player2 => &mut self.player2
        };
        
        PlayerGameData {player_id, player, stock_size,
                        trump_card_rank, public_data}
    }

    fn remove_card_from_hand(&mut self, player: Players, card: Card) {
        let player = self.get_player_mut(player);

        let index_option = player.hand.iter().position(
            |&card_in_hand| card_in_hand == card);
        index_option.map(|index| player.hand.remove(index));
    }

    fn add_cards_to_wins(&mut self, player: Players, cards: &[Card]) {
        let player_wins = &mut self.get_player_mut(player).wins;
        player_wins.extend_from_slice(cards);
    }

    fn deal_if_not_closed_or_empty(&mut self, winner_of_trick: Players) {
        if !self.is_closed() {            
            if let Some(card) = self.stock.pop() {
                let winning_player = self.get_player_mut(winner_of_trick);
                winning_player.hand.push(card);
            };

            if let Some(card) = self.stock.pop() {
                let losing_player
                    = self.get_player_mut(winner_of_trick.other());
                losing_player.hand.push(card);
            };
        }
    }
}

#[cfg(test)]
mod tests;

