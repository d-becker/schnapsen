use rand;
use rand::Rng;

use cards::{Card, Suit, Rank};
use schnapsen::{ErrorKind, player::Player};
use schnapsen::{generate_deck, first_beats_second, value};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Players {
    Player1,
    Player2
}

impl Players {
    pub fn other(&self) -> Players {
        match *self {
            Players::Player1 => Players::Player2,
            Players::Player2 => Players::Player1
        }
    }
}

#[derive(Debug)]
pub struct Game {
    stock: Vec<Card>,
    trump: Suit,
    closed: bool,
    game_over: bool,

    player1: Player,
    player2: Player,
    player_on_lead: Players,
    first_card_in_trick: Option<Card>
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

        Game {stock,
              trump,
              closed: false,
              game_over: false,
              player1: Player {name: "Player1".to_string(),
                               hand: hand1, ..Default::default()},
              player2: Player {name: "Player2".to_string(),
                               hand: hand2, ..Default::default()},
              player_on_lead: Players::Player1,
              first_card_in_trick: None
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
        self.player_on_lead
    }
    
    pub fn player_on_turn(&self) -> Players {
        match self.first_card_in_trick {
            Some(_) => self.player_on_lead.other(),
            None => self.player_on_lead
        }
    }
    
    pub fn trump_card(&self) -> Option<Card> {
        if self.closed {
            None
        } else {
            self.stock.first().map(|&card| card)
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn can_close(&self, player: Players) -> Result<(), ErrorKind> {
        self.on_turn(player)?;
        self.on_lead(player)?;
        
        if self.is_game_over() {
            Err(ErrorKind::GameOver)
        } else if self.is_closed() {
            Err(ErrorKind::DeckClosed)
        } else if self.stock.len() <= 2 {
            Err(ErrorKind::NotEnoughCardsInStock)
        } else {
            Ok(())
        }
    }
    
    pub fn close(&mut self, player: Players) -> Result<(), ErrorKind> {
        let can_close = self.can_close(player);

        if can_close.is_ok() {
            self.closed = true;
        }
        
        can_close
    }

    pub fn can_exchange_trump(&self, player: Players) -> Result<(), ErrorKind> {
        self.on_turn(player)?;
        self.on_lead(player)?;
        
        if self.is_game_over() {
            Err(ErrorKind::GameOver)
        } else if self.is_closed() {
            Err(ErrorKind::DeckClosed)
        } else if self.stock.len() <= 2 {
            Err(ErrorKind::NotEnoughCardsInStock)
        } else {
            let current_player = self.get_player(player);
        
            let trump_unter = Card::new(self.trump, Rank::Unter);
            if current_player.hand.contains(&trump_unter) {
                Ok(())
            } else {
                Err(ErrorKind::NoSuchCardInHand(trump_unter))
            }
        }
    }

    pub fn exchange_trump(&mut self, player: Players) -> Result<(), ErrorKind> {
        let can_exchange_trump = self.can_exchange_trump(player);
        if can_exchange_trump.is_ok() {
            let trump = self.trump;
            let trump_card = self.trump_card().unwrap();
            
            {
                let current_player = self.get_player_mut(player);
                
                let index = current_player.hand.iter()
                    .position(|&card| card == Card::new(trump, Rank::Unter))
                    .unwrap();
                current_player.hand[index] = trump_card;
            }
            
            self.stock[0] = Card::new(trump, Rank::Unter);
        }

        can_exchange_trump
    }

    pub fn can_call_twenty(&self, player: Players, suit: Suit)
                           -> Result<(), ErrorKind> {
        self.on_turn(player)?;
        self.on_lead(player)?;
        
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let current_player = self.get_player(player);

        let ober = Card::new(suit, Rank::Ober);
        let king = Card::new(suit, Rank::King);
        
        if !current_player.hand.contains(&ober) {
            Err(ErrorKind::NoSuchCardInHand(ober))
        } else if !current_player.hand.contains(&king) {
            Err(ErrorKind::NoSuchCardInHand(king))
        } else if current_player.twenties.contains(&suit) {
            Err(ErrorKind::AlreadyCalledThisTwenty(suit))
        } else if suit == self.trump {
            Err(ErrorKind::TwentyWithTrumpSuit)
        } else {
            Ok(())
        }
    }

    pub fn call_twenty(&mut self, player: Players, suit: Suit)
                       -> Result<(), ErrorKind> {
        let can_call_twenty = self.can_call_twenty(player, suit);

        if can_call_twenty.is_ok() {
            let current_player = self.get_player_mut(player);

            current_player.twenties.push(suit);
        }
        
        can_call_twenty
    }

    pub fn can_call_forty(&self, player: Players) -> Result<(), ErrorKind> {
        self.on_turn(player)?;
        self.on_lead(player)?;
        
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let current_player = self.get_player(player);

        let ober = Card::new(self.trump, Rank::Ober);
        let king = Card::new(self.trump, Rank::King);
        
        if !current_player.hand.contains(&ober) {
            Err(ErrorKind::NoSuchCardInHand(ober))
        } else if !current_player.hand.contains(&king) {
            Err(ErrorKind::NoSuchCardInHand(king))
        } else if current_player.forty.is_some() {
            Err(ErrorKind::AlreadyCalledForty)
        } else {
            Ok(())
        }
    }

    pub fn call_forty(&mut self, player: Players) -> Result<(), ErrorKind> {
        let can_call_forty = self.can_call_forty(player);
        if can_call_forty.is_ok() {
            let trump = self.trump;
            let current_player = self.get_player_mut(player);

            current_player.forty = Some(trump);
        }

        can_call_forty
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn can_declare_win(&self, player: Players) -> Result<(), ErrorKind> {
        self.on_turn(player)?;
        self.on_lead(player)?;
        
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let current_player = self.get_player(player);

        let player_score = current_player.score();
        
        if player_score < 66 {
            Err(ErrorKind::ScoreTooLow(player_score))
        } else {
            Ok(())
        }
    }
    
    pub fn declare_win(&mut self, player: Players) -> Result<(), ErrorKind> {
        let can_declare_win = self.can_declare_win(player);
        if can_declare_win.is_ok() {
            self.game_over = true;
        }

        can_declare_win
    }

    pub fn can_play_card(&self, player: Players, card: Card)
                         -> Result<(), ErrorKind> {
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        self.on_turn(player)?;
        
        if !self.get_player(player).hand.contains(&card) {
            return Err(ErrorKind::NoSuchCardInHand(card));
        }

        match self.first_card_in_trick {
            // We are playing the first card.
            None => Ok(()),

            // We are playing the second card.
            Some(first_card)
                => self.can_play_card_as_2nd_player(first_card, player, card)
        }
    }

    pub fn play_card(&mut self, player: Players, card: Card)
                     -> Result<(), ErrorKind> {
        self.can_play_card(player, card)?;

        self.remove_card_from_hand(player, card);

        match self.first_card_in_trick {
            // We are playing the first card in the trick.
            None => self.first_card_in_trick = Some(card),

            // We are playing the second card in the trick.
            Some(card_on_lead) => {
                let player_on_lead_wins
                    = first_beats_second(card_on_lead, card, self.trump);

                let winning_player_marker = if player_on_lead_wins
                {
                    self.player_on_lead
                } else {
                    player
                };

                self.add_cards_to_wins(winning_player_marker,
                                       &[card_on_lead, card]);

                self.deal_if_not_closed_or_empty(winning_player_marker);
                self.player_on_lead = winning_player_marker;
                self.first_card_in_trick = None;
            }
        }
        
        Ok(())
    }

    fn can_play_card_as_2nd_player(&self, first_card: Card,
                                   player: Players, card: Card)
                                   -> Result<(), ErrorKind> {
        let is_endgame = self.closed || self.stock.is_empty();
        
        if !is_endgame {
            return Ok(());
        }

        let player_on_lead = self.get_player(self.player_on_lead());
        let second_player = self.get_player(player);
        
        let legal_second_card = legal_second_card_in_endgame(
            &player_on_lead.hand, first_card,
            &second_player.hand, card,
            self.trump);
        
        legal_second_card
    }

    fn get_player_mut(&mut self, player: Players) -> &mut Player {
        match player {
            Players::Player1 => &mut self.player1,
            Players::Player2 => &mut self.player2
        }
    }

    fn on_lead(&self, player: Players) -> Result<(), ErrorKind> {
        if self.player_on_lead == player {
            Ok(())
        } else {
            Err(ErrorKind::PlayerNotOnLead)
        }
    }
    
    fn on_turn(&self, player: Players) -> Result<(), ErrorKind> {
        if self.player_on_lead == player {
            match self.first_card_in_trick {
                Some(_) => Err(ErrorKind::NotPlayersTurn),
                None => Ok(())
            }
        } else {
            match self.first_card_in_trick {
                Some(_) => Ok(()),
                None => Err(ErrorKind::NotPlayersTurn)
            }
        }
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
        if !self.closed {            
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

fn legal_second_card_in_endgame(_hand1: &[Card], card1: Card,
                                hand2: &[Card], card2: Card,
                                trump: Suit) -> Result<(), ErrorKind> {
    if card1.suit() == card2.suit() {
        if value(card1) < value(card2) {
            Ok(())
        } else {
            let better_card_position = hand2.iter()
                .position(|&other_card| other_card.suit() == card1.suit()
                          && value(card1) < value(other_card));
            
            match better_card_position {
                None => Ok(()),
                Some(index) =>Err(ErrorKind::MustTake(hand2[index]))
            }
        }
    } else {
        if hand2.iter().any(|&other_card| other_card.suit() == card1.suit()) {
            Err(ErrorKind::MustUseAnotherSuit(card1.suit()))
        } else if card2.suit() == trump {
            Ok(())
        } else if hand2.iter().any(|&other_card| other_card.suit() == trump) {
            Err(ErrorKind::MustUseTrump)
        } else {
            Ok(())
        }   
    }
    
}

#[cfg(test)]
mod tests;
