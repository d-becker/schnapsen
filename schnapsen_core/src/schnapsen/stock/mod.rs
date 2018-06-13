use cards::Card;

use std::vec::Vec;


pub trait IStock {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn trump_card(&self) -> Option<Card>; // TODO: Only the rank should be stored.
    fn exchange_trump_card(&mut self, card: Card) -> Option<Card>;
    
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

    fn trump_card(&self) -> Option<Card> {
        if self.is_closed() {
            None
        } else {
            self.data.cards.first().map(|&card| card)
        }
    }

    fn exchange_trump_card(&mut self, card: Card) -> Option<Card> {
        let old_trump_card_option = self.trump_card();

        if let Some(_) = old_trump_card_option {
            self.data.cards[0] = card;
            old_trump_card_option
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
        self.data.cards.pop()
    }
}

#[derive(Default, Debug)]
pub struct DummyStock {
    length: usize,
    trump_card: Option<Card>,
    closed: bool
}

impl DummyStock {
    pub fn new(length: usize, trump_card: Option<Card>, closed: bool)
               -> DummyStock {
        DummyStock {length, trump_card, closed}
    }
}

impl IStock for DummyStock {
    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn len(&self) -> usize {
        self.length
    }

    fn trump_card(&self) -> Option<Card> {
        if self.is_closed() || self.is_empty() {
            None
        } else {
            self.trump_card
        }
    }

    fn exchange_trump_card(&mut self, card: Card) -> Option<Card> {
        if let Some(trump_c) = self.trump_card() {
            self.trump_card = Some(card);
            Some(trump_c)
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
