use rand;
use rand::Rng;

use cards::{Card, Suit, Rank};
use schnapsen::{ErrorKind, Players, player::Player};
use schnapsen::{generate_deck, first_beats_second};

use super::game_data::PublicGameData;
use super::player_game::PlayerGame;

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
        Game::new_(deck).unwrap()
    }
}

impl Game {
    fn new_(mut deck: Vec<Card>) -> Option<Game> {
        let deck_length = deck.len();

        // Don't allow an odd number of cards in the deck.
        if deck_length % 2 == 1 {
            return None;
        }
        
        let hand1 = deck.split_off(deck_length - 5);
        
        let deck_length = deck.len();
        let hand2 = deck.split_off(deck_length - 5);

        let stock = deck;
        let trump = stock[0].suit();

        let public_data = PublicGameData {trump, closed: false, winner: None,
                                          player_on_lead: Players::Player1,
                                          first_card_in_trick: None};
        let game = Game {stock,
              player1: Player {name: "Player1".to_string(),
                               hand: hand1, ..Default::default()},
              player2: Player {name: "Player2".to_string(),
                               hand: hand2, ..Default::default()},
              public_data
        };

        Some(game)
    }
    
    pub fn new_random() -> Game {
        let mut deck = generate_deck();
        let mut rng = rand::isaac::IsaacRng::new_unseeded();
        rng.shuffle(&mut deck);

        Game::new_(deck).unwrap()
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

    pub fn can_play_card_twenty(&self, card: Card) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_play_card_twenty(card)
    }

    pub fn play_card_twenty(&mut self, card: Card) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let mut data_as_player_mut
            = self.get_data_as_player_mut(player_on_turn);
        data_as_player_mut.play_card_twenty(card)
    }

    pub fn can_play_card_forty(&self, card: Card) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_play_card_forty(card)
    }

    pub fn play_card_forty(&mut self, card: Card) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let mut data_as_player_mut
            = self.get_data_as_player_mut(player_on_turn);
        data_as_player_mut.play_card_forty(card)
    }

    pub fn can_play_card(&self, card: Card) -> Result<(), ErrorKind> {
        let player_on_turn = self.player_on_turn();
        let data_as_player = self.get_data_as_player(player_on_turn);
        data_as_player.can_play_card(card)
    }

    pub fn play_card(&mut self, card: Card)
                     -> Result<Option<(Card, Card)>, ErrorKind> {
        self.can_play_card(card)?;

        let player_on_turn = self.player_on_turn();

        self.remove_card_from_hand(player_on_turn, card);

        if let Some(card_on_lead) = self.public_data.first_card_in_trick  {
            // We are playing the second card in the trick.
            self.play_second_card(card, card_on_lead)
        } else {
            // We are playing the first card in the trick.
            self.play_first_card(card).map(|_| None)
        }
    }

    fn play_first_card(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.public_data.first_card_in_trick = Some(card);
        Ok(())
    }

    fn play_second_card(&mut self, card: Card, card_on_lead: Card)
                        -> Result<Option<(Card, Card)>, ErrorKind> {
        let player_on_turn = self.player_on_turn();
        
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

        let dealed_cards
            = self.deal_if_not_closed_or_empty(winning_player_marker);
        self.public_data.player_on_lead = winning_player_marker;
        self.public_data.first_card_in_trick = None;

        // Actually the players should always have
        // the same number of cards in their hands.
        if self.player1.hand.is_empty()
            || self.player2.hand.is_empty() {
                self.public_data.winner = Some(winning_player_marker);
        }

        Ok(dealed_cards)
    }

    fn get_player_mut(&mut self, player: Players) -> &mut Player {
        match player {
            Players::Player1 => &mut self.player1,
            Players::Player2 => &mut self.player2
        }
    }

    fn get_data_as_player(&self, player_id: Players)
                          -> PlayerGame<&Player, &PublicGameData> {
        let player = self.get_player(player_id);
        let stock_size = self.stock.len() as u32;
        let trump_card_rank = self.trump_card().map(|card| card.rank());
        let public_data = &self.public_data;
        PlayerGame {player_id, player, stock_size,
                    trump_card_rank, public_data}
    }

    fn get_data_as_player_mut(&mut self, player_id: Players)
                              -> PlayerGame<&mut Player, &mut PublicGameData>
    {
        let stock_size = self.stock.len() as u32;
        let trump_card_rank = self.trump_card().map(|card| card.rank());
        let public_data = &mut self.public_data;
        let player = match player_id {
            Players::Player1 => &mut self.player1,
            Players::Player2 => &mut self.player2
        };
        
        PlayerGame {player_id, player, stock_size,
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

    fn deal_if_not_closed_or_empty(&mut self, winner_of_trick: Players)
                                   -> Option<(Card, Card)> {
        if self.is_closed() || self.stock.is_empty() {
            return None;
        }

        // We can safely unwrap these because we always have an even
        // number of cards in the stock and the stock is not empty.
        let winner_new_card = self.stock.pop().unwrap();
        let loser_new_card = self.stock.pop().unwrap();
        
        {
            let winning_player = self.get_player_mut(winner_of_trick);
            winning_player.hand.push(winner_new_card);
        }

        {
            let losing_player
                = self.get_player_mut(winner_of_trick.other());
            losing_player.hand.push(loser_new_card);
        }

        if winner_of_trick == Players::Player1 {
            Some((winner_new_card, loser_new_card))
        } else {
            Some((loser_new_card, winner_new_card))
        }
    }
}

#[cfg(test)]
mod tests;

