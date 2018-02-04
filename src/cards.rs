#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Suite {
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
    pub suite: Suite,
    pub rank: Rank
}

impl Card {
    pub fn new(suite: Suite, rank: Rank) -> Card {
        Card {suite, rank}
    }
    
    pub fn suite(&self) -> Suite {
        self.suite
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}
