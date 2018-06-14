pub mod client_game;
pub mod server_game;

use cards::{Card, Suit, Rank};

use schnapsen::{first_beats_second, value};
use schnapsen::ErrorKind;
use schnapsen::player::{IPlayer, PlayerId};
use schnapsen::stock::IStock;

#[derive(Debug)]
pub struct Game<STOCK: IStock> {
    stock: STOCK,
    player1: Box<IPlayer>,
    player2: Box<IPlayer>,
    
    trump: Suit,
    winner: Option<PlayerId>,
    player_on_lead: PlayerId,
    first_card_in_trick: Option<Card>
}

impl<STOCK> Game<STOCK> where STOCK: IStock {
    pub fn get_stock(&self) -> &STOCK {
        &self.stock
    }
    
    pub fn get_player1(&self) -> &Box<IPlayer> {
        &self.player1
    }

    pub fn get_player2(&self) -> &Box<IPlayer> {
        &self.player2
    }

    pub fn get_player(&self, player: PlayerId) -> &Box<IPlayer> {
        match player {
            PlayerId::Player1 => self.get_player1(),
            PlayerId::Player2 => self.get_player2()
        }
    }

    pub fn player_id_on_lead(&self) -> PlayerId {
        self.player_on_lead
    }
    
    pub fn player_id_on_turn(&self) -> PlayerId {
        match self.first_card_in_trick {
            Some(_) => self.player_on_lead.other(),
            None => self.player_on_lead
        }
    }

    pub fn player_on_turn(&self) -> &Box<IPlayer> {
        let player_id = self.player_id_on_turn();
        self.get_player(player_id)
    }

    pub fn trump(&self) -> Suit {
        self.trump
    }
    
    pub fn trump_card(&self) -> Option<Card> {
        self.stock.trump_card_rank().map(|rank| Card::new(self.trump(), rank))
    }

    pub fn get_first_card_in_trick(&self) -> Option<Card> {
        self.first_card_in_trick
    }

    pub fn is_closed(&self) -> bool {
        self.stock.is_closed()
    }

    pub fn can_close(&self) -> Result<(), ErrorKind> {
        self.on_lead()?;

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
    
    pub fn close(&mut self) -> Result<(), ErrorKind> {
        self.can_close()?;

        self.stock.close();

        Ok(())
    }

    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }

    pub fn winner(&self) -> Option<PlayerId> {
        self.winner
    }

    pub fn can_exchange_trump(&self) -> Result<(), ErrorKind> {
        self.on_lead()?;
       
        if self.is_game_over() {
            Err(ErrorKind::GameOver)
        } else if self.is_closed() {
            Err(ErrorKind::DeckClosed)
        } else if self.stock.len() <= 2 {
            Err(ErrorKind::NotEnoughCardsInStock)
        } else {
            let player = self.player_on_turn();
            let trump_unter = Card::new(self.trump(), Rank::Unter);
            if player.get_hand().contains(&trump_unter) {
                Ok(())
            } else {
                Err(ErrorKind::NoSuchCardInHand(trump_unter))
            }
        }
    }

    pub fn exchange_trump(&mut self) -> Result<(), ErrorKind> {
        self.can_exchange_trump()?;

        let old_trump_card_rank
            = self.stock.exchange_trump_card(Rank::Unter).unwrap();
        let old_trump_card = Card::new(self.trump(), old_trump_card_rank);
        
        let trump_unter = Card::new(self.trump(), Rank::Unter);
        let player = self.player_on_turn_mut();
        player.remove_from_hand(trump_unter);
        player.add_to_hand(old_trump_card);

        Ok(())
    }

    pub fn can_declare_win(&self) -> Result<(), ErrorKind> {
        self.on_lead()?;
        
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player_on_turn();;

        let player_score = player.score();
        
        if player_score < 66 {
            Err(ErrorKind::ScoreTooLow(player_score))
        } else {
            Ok(())
        }
    }
    
    pub fn declare_win(&mut self) -> Result<(), ErrorKind> {
        self.can_declare_win()?;
        
        self.winner = Some(self.player_id_on_turn());

        Ok(())
    }

    pub fn can_play_card_twenty(&self, card: Card) -> Result<(), ErrorKind> {
        self.on_lead()?;

        if card.rank() != Rank::Ober && card.rank() != Rank::King {
            return Err(ErrorKind::NotTwentyCard(card));
        }

        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player_on_turn();

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
        } else if suit == self.trump() {
            Err(ErrorKind::TwentyWithTrumpSuit)
        } else {
            Ok(())
        }
    }

    pub fn play_card_twenty(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card_twenty(card)?;

        {
            let player = self.player_on_turn_mut();
            player.add_twenty(card.suit());
        }

        self.play_card(card).map(|_| ())
    }

    pub fn can_declare_twenty_win(&self, suit: Suit) -> Result<(), ErrorKind> {
        self.can_play_card_twenty(Card::new(suit, Rank::King))?;

        let score = self.player_on_turn().score();

        if score >= 46 {
            Ok(())
        } else {
            Err(ErrorKind::ScoreTooLow(score))
        }
    }

    pub fn declare_twenty_win(&mut self, suit: Suit) -> Result<(), ErrorKind> {
        self.can_declare_twenty_win(suit)?;
        
        self.winner = Some(self.player_id_on_turn());

        Ok(())
    }

    pub fn can_play_card_forty(&self, card: Card) -> Result<(), ErrorKind> {
        self.on_lead()?;

        if card.rank() != Rank::Ober && card.rank() != Rank::King {
            return Err(ErrorKind::NotFortyCard(card));
        }
        
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player_on_turn();

        let ober = Card::new(self.trump(), Rank::Ober);
        let king = Card::new(self.trump(), Rank::King);
        
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

    pub fn play_card_forty(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.can_play_card_forty(card)?;

        {
            let trump = self.trump();
            let player = self.player_on_turn_mut();

            player.add_forty(trump);
        }

        self.play_card(card).map(|_| ())
    }

    pub fn can_declare_forty_win(&self) -> Result<(), ErrorKind> {
        self.can_play_card_forty(Card::new(self.trump(), Rank::King))?;

        let score = self.player_on_turn().score();

        if score >= 26 {
            Ok(())
        } else {
            Err(ErrorKind::ScoreTooLow(score))
        }
    }

    pub fn declare_forty_win(&mut self) -> Result<(), ErrorKind> {
        self.can_declare_forty_win()?;
        
        self.winner = Some(self.player_id_on_turn());

        Ok(())
    }

    pub fn can_play_card(&self, card: Card) -> Result<(), ErrorKind> {
        if self.is_game_over() {
            return Err(ErrorKind::GameOver);
        }
        
        let player = self.player_on_turn();
        
        if !player.get_hand().contains(&card) {
            return Err(ErrorKind::NoSuchCardInHand(card));
        }

        match self.first_card_in_trick {
            // We are playing the first card.
            None => Ok(()),

            // We are playing the second card.
            Some(first_card) => self.can_play_card_as_2nd_player(
                first_card, card)
        }
    }

    
    pub fn play_card(&mut self, card: Card)
                     -> Result<Option<(Card, Card)>, ErrorKind> {
        self.can_play_card(card)?;

        self.player_on_turn_mut().remove_from_hand(card);

        if let Some(card_on_lead) = self.first_card_in_trick  {
            // We are playing the second card in the trick.
            self.play_second_card(card, card_on_lead)
        } else {
            // We are playing the first card in the trick.
            self.play_first_card(card).map(|_| None)
        }
    }

    fn play_first_card(&mut self, card: Card) -> Result<(), ErrorKind> {
        self.first_card_in_trick = Some(card);
        Ok(())
    }

    fn play_second_card(&mut self, card: Card, card_on_lead: Card)
                        -> Result<Option<(Card, Card)>, ErrorKind> {        
        let player_on_lead_wins
            = first_beats_second(card_on_lead, card, self.trump());

        let winning_player_id = if player_on_lead_wins
        {
            self.player_id_on_lead()
        } else {
            self.player_id_on_turn()
        };

        {
            let winning_player = self.get_player_mut(winning_player_id);
            winning_player.add_to_wins(card_on_lead, card);
        }

        let dealed_cards
            = self.deal_if_not_closed_or_empty(winning_player_id);
        self.player_on_lead = winning_player_id;
        self.first_card_in_trick = None;

        // Actually the players should always have
        // the same number of cards in their hands.
        if self.player1.get_hand().is_empty()
            || self.player2.get_hand().is_empty() {
                self.winner = Some(winning_player_id);
            }

        Ok(dealed_cards)
    }

    fn can_play_card_as_2nd_player(&self, first_card: Card, card: Card)
                                   -> Result<(), ErrorKind> {
        let is_endgame = self.is_closed() || self.stock.len() == 0;
        
        if !is_endgame {
            return Ok(());
        }

        let player = self.player_on_turn();
        
        let legal_second_card = legal_second_card_in_endgame(
            first_card, player.get_hand(),
            card, self.trump());
        
        legal_second_card
    }

    fn get_player_mut(&mut self, player: PlayerId) -> &mut Box<IPlayer> {
        match player {
            PlayerId::Player1 => &mut self.player1,
            PlayerId::Player2 => &mut self.player2
        }
    }

    fn player_on_turn_mut(&mut self) -> &mut Box<IPlayer> {
        let player_id = self.player_id_on_turn();
        self.get_player_mut(player_id)
    }

    fn on_lead(&self) -> Result<(), ErrorKind> {
        if self.player_id_on_lead() == self.player_id_on_turn() {
            Ok(())
        } else {
            Err(ErrorKind::PlayerNotOnLead)
        }
    }

    fn deal_if_not_closed_or_empty(&mut self, winner_of_trick: PlayerId)
                                   -> Option<(Card, Card)> {
        if self.is_closed() || self.stock.is_empty() {
            return None;
        }

        let winner_new_card_option = self.stock.deal();
        let loser_new_card_option = self.stock.deal();
        
        if let (Some(winner_new_card), Some(loser_new_card))
            = (winner_new_card_option, loser_new_card_option)
        {
            {
                let winning_player = self.get_player_mut(winner_of_trick);
                winning_player.add_to_hand(winner_new_card);
            };

            {
                let losing_player
                    = self.get_player_mut(winner_of_trick.other());
                losing_player.add_to_hand(loser_new_card);
            };

            if winner_of_trick == PlayerId::Player1 {
                Some((winner_new_card, loser_new_card))
            } else {
                Some((loser_new_card, winner_new_card))
            }
        } else {
            None
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

#[cfg(test)]
mod tests;

