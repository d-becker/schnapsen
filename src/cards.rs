#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Suit {
    Hearts,
    Bells,
    Acorns,
    Leaves
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
