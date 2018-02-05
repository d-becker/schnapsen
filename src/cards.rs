#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Suit {
    Hearts,
    Bells,
    Acorns,
    Leaves
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Ten,
    Unter,
    Ober,
    King,
    Ace
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {suit, rank}
    }
    
    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}
