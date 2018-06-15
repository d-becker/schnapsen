use cards::{Card, Rank};

use std::vec::Vec;

pub trait IStock {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn trump_card_rank(&self) -> Option<Rank>;
    fn exchange_trump_card(&mut self, rank: Rank) -> Option<Rank>;
    
    fn is_closed(&self) -> bool;
    fn close(&mut self);

    fn deal(&mut self) -> Option<Card>;
}

#[derive(Default, Debug)]
pub struct StockData {
    pub cards: Vec<Card>,
    pub closed: bool
}

#[derive(Default, Debug)]
pub struct Stock {
    data: StockData
}

impl Stock {
    pub fn new(data: StockData) -> Stock {
        Stock {data}
    }
}

impl IStock for Stock {
    fn is_empty(&self) -> bool {
        self.data.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.data.cards.len()
    }

    fn trump_card_rank(&self) -> Option<Rank> {
        if self.is_closed() {
            None
        } else {
            self.data.cards.first().map(|&card| card.rank())
        }
    }

    fn exchange_trump_card(&mut self, rank: Rank) -> Option<Rank> {
        let old_trump_card_rank_option = self.trump_card_rank();

        if let Some(_) = old_trump_card_rank_option {
            let trump = self.data.cards[0].suit();
            self.data.cards[0] = Card::new(trump, rank);
            old_trump_card_rank_option
        } else {
            None
        }
    }
    
    fn is_closed(&self) -> bool {
        self.data.closed
    }

    fn close(&mut self) {
        self.data.closed = true;
    }

    fn deal(&mut self) -> Option<Card> {
        if !self.is_closed() {
            self.data.cards.pop()
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
pub struct DummyStock {
    length: usize,
    trump_card_rank: Option<Rank>,
    closed: bool
}

impl DummyStock {
    pub fn new(length: usize, trump_card_rank: Option<Rank>, closed: bool)
               -> DummyStock {
        DummyStock {length, trump_card_rank, closed}
    }
}

impl IStock for DummyStock {
    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn len(&self) -> usize {
        self.length
    }

    fn trump_card_rank(&self) -> Option<Rank> {
        if self.is_closed() || self.is_empty() {
            None
        } else {
            self.trump_card_rank
        }
    }

    fn exchange_trump_card(&mut self, rank: Rank) -> Option<Rank> {
        if let Some(trump_c_r) = self.trump_card_rank() {
            self.trump_card_rank = Some(rank);
            Some(trump_c_r)
        } else {
            None
        }
    }
    
    fn is_closed(&self) -> bool {
        self.closed
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn deal(&mut self) -> Option<Card> {
        if !self.is_empty() {
            self.length -= 1;
        }

        None
    }
}

#[cfg(test)]
mod tests;
