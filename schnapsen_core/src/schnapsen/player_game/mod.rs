use std::borrow::{Borrow, BorrowMut};

use cards::{Card, Suit, Rank};

use schnapsen::ErrorKind;
use schnapsen::{first_beats_second, value};
use schnapsen::game_data::PublicGameData;
use schnapsen::player::{IPlayer, Player, PlayerId};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerGame<P, D>
    where P: Borrow<Player>,
          D: Borrow<PublicGameData>
{
    pub player_id: PlayerId,
    pub player: P,
    pub stock_size: u32,
    pub trump_card_rank: Option<Rank>,
    pub public_data: D
}

impl<P, D> PlayerGame<P, D>
    where P: Borrow<Player>,
          D: Borrow<PublicGameData>
{
    pub fn player_on_lead(&self) -> PlayerId {
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
            if player.get_hand().contains(&trump_unter) {
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
        
        if !player.get_hand().contains(&card) {
            Err(ErrorKind::NoSuchCardInHand(card))
        } else if !player.get_hand().contains(&other_card_in_twenty) {
            Err(ErrorKind::NoSuchCardInHand(other_card_in_twenty))
        } else if player.get_twenties().contains(&suit) {
            Err(ErrorKind::AlreadyCalledThisTwenty(suit))
        } else if suit == borrowed_data.trump {
            Err(ErrorKind::TwentyWithTrumpSuit)
        } else {
            Ok(())
        }
    }

    pub fn can_declare_twenty_win(&self, suit: Suit) -> Result<(), ErrorKind> {
        self.can_play_card_twenty(Card::new(suit, Rank::King))?;

        let score = self.player.borrow().score();

        if score >= 46 {
            Ok(())
        } else {
            Err(ErrorKind::ScoreTooLow(score))
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
        
        if !player.get_hand().contains(&ober) {
            Err(ErrorKind::NoSuchCardInHand(ober))
        } else if !player.get_hand().contains(&king) {
            Err(ErrorKind::NoSuchCardInHand(king))
        } else if player.get_forty().is_some() {
            Err(ErrorKind::AlreadyCalledForty)
        } else {
            Ok(())
        }
    }

    pub fn can_declare_forty_win(&self) -> Result<(), ErrorKind> {
        let trump = self.public_data.borrow().trump;
        self.can_play_card_forty(Card::new(trump, Rank::King))?;

        let score = self.player.borrow().score();

        if score >= 26 {
            Ok(())
        } else {
            Err(ErrorKind::ScoreTooLow(score))
        }
    }

    pub fn can_play_card(&self, card: Card) -> Result<(), ErrorKind> {
        self.on_turn()?;

        let borrowed_data = self.public_data.borrow();
        
        if borrowed_data.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player.borrow();
        
        if !player.get_hand().contains(&card) {
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
            first_card, player.get_hand(),
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
            Err(ErrorKind::NotPlayersTurn)
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

                player.remove_from_hand(Card::new(trump, Rank::Unter));
                player.add_to_hand(trump_card);
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
            player.add_twenty(card.suit());
        }

        self.play_card(card).map(|_| ())
    }

    pub fn declare_twenty_win(&mut self, suit: Suit) -> Result<(), ErrorKind> {
        let can_declare_twenty_win = self.can_declare_twenty_win(suit);
        if can_declare_twenty_win.is_ok() {
            self.public_data.borrow_mut().winner = Some(self.player_id);
        }

        can_declare_twenty_win
    }

    pub fn play_card_forty(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card_forty(card)?;

        {
            let trump = self.public_data.borrow().trump;
            let player = self.player.borrow_mut();

            player.add_forty(trump);
        }

        self.play_card(card).map(|_| ())
    }

    pub fn declare_forty_win(&mut self) -> Result<(), ErrorKind> {
        let can_declare_forty_win = self.can_declare_forty_win();
        if can_declare_forty_win.is_ok() {
            self.public_data.borrow_mut().winner = Some(self.player_id);
        }

        can_declare_forty_win
    }

    // Returns the winner of the trick and the first card in the trick, if this
    // is the second card in the trick.
    pub fn play_card(&mut self, card: Card)
                     -> Result<Option<(PlayerId, Card)>, ErrorKind> {
        self.can_play_card(card)?;

        self.player.borrow_mut().remove_from_hand(card);

        let borrowed_data = self.public_data.borrow_mut();
        match borrowed_data.first_card_in_trick {
            // We are playing the first card in the trick.
            None => {
                borrowed_data.first_card_in_trick = Some(card);
                Ok(None)
            },

            // We are playing the second card in the trick.
            Some(card_on_lead) => {
                let player_on_lead_wins
                    = first_beats_second(card_on_lead, card,
                                         borrowed_data.trump);

                let winning_player_id = if player_on_lead_wins
                {
                    self.player_id.other()
                } else {
                    self.player_id
                };

                if self.player_id == winning_player_id {
                    let player = self.player.borrow_mut();
                    player.add_to_wins(card_on_lead, card);
                }

                borrowed_data.player_on_lead = winning_player_id;
                borrowed_data.first_card_in_trick = None;

                let will_deal = !borrowed_data.is_closed()
                    && self.stock_size > 0;
                
                let hand_empty = self.player.borrow().get_hand().is_empty();
                if !will_deal && hand_empty {
                    borrowed_data.winner = Some(winning_player_id);
                }

                if will_deal {
                    self.stock_size -= 2;
                }

                Ok(Some((winning_player_id, card_on_lead)))
            }
        }
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
