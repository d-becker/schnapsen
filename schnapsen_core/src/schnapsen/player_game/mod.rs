use std::borrow::{Borrow, BorrowMut};

use cards::{Card, Suit, Rank};
use schnapsen::{ErrorKind, Players, player::Player};
use schnapsen::{first_beats_second, value};
use schnapsen::game_data::PublicGameData;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerGame<P, D>
    where P: Borrow<Player>,
          D: Borrow<PublicGameData>
{
    pub player_id: Players,
    pub player: P,
    pub stock_size: u32,
    pub trump_card_rank: Option<Rank>,
    pub public_data: D
}

impl<P, D> PlayerGame<P, D>
    where P: Borrow<Player>,
          D: Borrow<PublicGameData>
{
    pub fn player_on_lead(&self) -> Players {
        self.public_data.borrow().player_on_lead()
    }
    
    pub fn can_close(&self) -> Result<(), ErrorKind> {
        self.on_turn()?;
        self.on_lead()?;
        
        let borrowed_data = self.public_data.borrow();
        if borrowed_data.is_game_over() {
            Err(ErrorKind::GameOver)
        } else if borrowed_data.is_closed() {
            Err(ErrorKind::DeckClosed)
        } else if self.stock_size <= 2 {
            Err(ErrorKind::NotEnoughCardsInStock)
        } else {
            Ok(())
        }
    }

    pub fn can_exchange_trump(&self) -> Result<(), ErrorKind> {
        self.on_turn()?;
        self.on_lead()?;
        
        let borrowed_data = self.public_data.borrow();        
        if borrowed_data.is_game_over() {
            Err(ErrorKind::GameOver)
        } else if borrowed_data.is_closed() {
            Err(ErrorKind::DeckClosed)
        } else if self.stock_size <= 2 {
            Err(ErrorKind::NotEnoughCardsInStock)
        } else {
            let player = self.player.borrow();
            let trump_unter = Card::new(borrowed_data.trump, Rank::Unter);
            if player.hand.contains(&trump_unter) {
                Ok(())
            } else {
                Err(ErrorKind::NoSuchCardInHand(trump_unter))
            }
        }
    }

    pub fn can_declare_win(&self) -> Result<(), ErrorKind> {
        self.on_turn()?;
        self.on_lead()?;

        let borrowed_data = self.public_data.borrow();
        
        if borrowed_data.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player.borrow();

        let player_score = player.score();
        
        if player_score < 66 {
            Err(ErrorKind::ScoreTooLow(player_score))
        } else {
            Ok(())
        }
    }

    pub fn can_play_card_twenty(&self, card: Card) -> Result<(), ErrorKind> {
        self.on_turn()?;
        self.on_lead()?;

        if card.rank() != Rank::Ober && card.rank() != Rank::King {
            return Err(ErrorKind::NotTwentyCard(card));
        }

        let borrowed_data = self.public_data.borrow();
        if borrowed_data.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player.borrow();

        let other_rank_in_twenty = if card.rank() == Rank::Ober {
            Rank::King
        } else {
            Rank::Ober
        };

        let suit = card.suit();
        let other_card_in_twenty = Card::new(suit, other_rank_in_twenty);
        
        if !player.hand.contains(&card) {
            Err(ErrorKind::NoSuchCardInHand(card))
        } else if !player.hand.contains(&other_card_in_twenty) {
            Err(ErrorKind::NoSuchCardInHand(other_card_in_twenty))
        } else if player.twenties.contains(&suit) {
            Err(ErrorKind::AlreadyCalledThisTwenty(suit))
        } else if suit == borrowed_data.trump {
            Err(ErrorKind::TwentyWithTrumpSuit)
        } else {
            Ok(())
        }
    }

    pub fn can_play_card_forty(&self, card: Card) -> Result<(), ErrorKind> {
        self.on_turn()?;
        self.on_lead()?;

        if card.rank() != Rank::Ober && card.rank() != Rank::King {
            return Err(ErrorKind::NotFortyCard(card));
        }

        let borrowed_data = self.public_data.borrow();
        
        if borrowed_data.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player.borrow();

        let ober = Card::new(borrowed_data.trump, Rank::Ober);
        let king = Card::new(borrowed_data.trump, Rank::King);
        
        if !player.hand.contains(&ober) {
            Err(ErrorKind::NoSuchCardInHand(ober))
        } else if !player.hand.contains(&king) {
            Err(ErrorKind::NoSuchCardInHand(king))
        } else if player.forty.is_some() {
            Err(ErrorKind::AlreadyCalledForty)
        } else {
            Ok(())
        }
    }

    pub fn can_play_card(&self, card: Card) -> Result<(), ErrorKind> {
        self.on_turn()?;

        let borrowed_data = self.public_data.borrow();
        
        if borrowed_data.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player.borrow();
        
        if !player.hand.contains(&card) {
            return Err(ErrorKind::NoSuchCardInHand(card));
        }

        match borrowed_data.first_card_in_trick {
            // We are playing the first card.
            None => Ok(()),

            // We are playing the second card.
            Some(first_card) => self.can_play_card_as_2nd_player(
                first_card, card)
        }
    }

    fn can_play_card_as_2nd_player(&self, first_card: Card, card: Card)
                                   -> Result<(), ErrorKind> {
        let borrowed_data = self.public_data.borrow();
        let is_endgame = borrowed_data.is_closed()
            || self.stock_size == 0;
        
        if !is_endgame {
            return Ok(());
        }

        let player = self.player.borrow();
        
        let legal_second_card = legal_second_card_in_endgame(
            first_card, &player.hand,
            card, borrowed_data.trump);
        
        legal_second_card
    }
    
    fn on_lead(&self) -> Result<(), ErrorKind> {
        if self.player_on_lead() == self.player_id {
            Ok(())
        } else {
            Err(ErrorKind::PlayerNotOnLead)
        }
    }

    fn on_turn(&self) -> Result<(), ErrorKind> {
        if self.public_data.borrow().player_on_turn() == self.player_id {
            Ok(())
        } else {
            Err(ErrorKind::PlayerNotOnLead)
        }
    }
}

impl<P, D> PlayerGame<P, D>
    where P: BorrowMut<Player>,
          D: BorrowMut<PublicGameData>
{
    pub fn close(&mut self) -> Result<(), ErrorKind> {
        let can_close = self.can_close();

        if can_close.is_ok() {
            self.public_data.borrow_mut().closed = true;
        }
        
        can_close
    }

    pub fn exchange_trump(&mut self) -> Result<(), ErrorKind> {
        let borrowed_data = self.public_data.borrow();
        
        let can_exchange_trump = self.can_exchange_trump();
        if can_exchange_trump.is_ok() {
            let trump = borrowed_data.trump;
            let trump_card = Card::new(trump, self.trump_card_rank.unwrap());
            
            {
                let player = self.player.borrow_mut();
                
                let index = player.hand.iter()
                    .position(|&card| card == Card::new(trump, Rank::Unter))
                    .unwrap();
                player.hand[index] = trump_card;
            }
        }

        can_exchange_trump
    }

    pub fn declare_win(&mut self) -> Result<(), ErrorKind> {
        let can_declare_win = self.can_declare_win();
        if can_declare_win.is_ok() {
            self.public_data.borrow_mut().winner = Some(self.player_id);
        }

        can_declare_win
    }

    pub fn play_card_twenty(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card_twenty(card)?;

        {
            let player = self.player.borrow_mut();
            player.twenties.push(card.suit());
        }

        self.play_card(card).map(|_| ())
    }

    pub fn play_card_forty(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card_forty(card)?;

        {
            let trump = self.public_data.borrow().trump;
            let player = self.player.borrow_mut();

            player.forty = Some(trump);
        }

        self.play_card(card).map(|_| ())
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card(card)?;

        self.remove_card_from_hand(card);

        let borrowed_data = self.public_data.borrow_mut();
        match borrowed_data.first_card_in_trick {
            // We are playing the first card in the trick.
            None => borrowed_data.first_card_in_trick = Some(card),

            // We are playing the second card in the trick.
            Some(card_on_lead) => {
                let player_on_lead_wins
                    = first_beats_second(card_on_lead, card,
                                         borrowed_data.trump);

                let winning_player_marker = if player_on_lead_wins
                {
                    self.player_id.other()
                } else {
                    self.player_id
                };

                if self.player_id == winning_player_marker {
                    let player = self.player.borrow_mut();
                    player.wins.extend_from_slice(&[card_on_lead, card]);
                }

                borrowed_data.player_on_lead = winning_player_marker;
                borrowed_data.first_card_in_trick = None;

                self.stock_size -= 2;
                let hand_empty = self.player.borrow().hand.is_empty();
                if self.stock_size == 0 && hand_empty {
                    borrowed_data.winner = Some(winning_player_marker);
                }
            }
        }
        
        Ok(())
    }

    fn remove_card_from_hand(&mut self, card: Card) {
        let player = self.player.borrow_mut();
        
        let index_option = player.hand.iter().position(
            |&card_in_hand| card_in_hand == card);
        index_option.map(|index| player.hand.remove(index));
    }
}


fn legal_second_card_in_endgame(card1: Card,
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
