use super::*;

#[test]
fn test_value_unter() {
    let card = Card::new(Suit::Hearts, Rank::Unter);
    assert_eq!(2, value(card));
}

#[test]
fn test_value_ober() {
    let card = Card::new(Suit::Bells, Rank::Ober);
    assert_eq!(3, value(card));
}

#[test]
fn test_value_king() {
    let card = Card::new(Suit::Leaves, Rank::King);
    assert_eq!(4, value(card));
}

#[test]
fn test_value_ten() {
    let card = Card::new(Suit::Hearts, Rank::Ten);
    assert_eq!(10, value(card));
}

#[test]
fn test_value_ace() {
    let card = Card::new(Suit::Acorns, Rank::Ace);
    assert_eq!(11, value(card));
}

#[test]
fn test_value_illegal_card() {
    let card = Card::new(Suit::Acorns, Rank::Seven);
    assert_eq!(0, value(card));
}

#[test]
fn test_generate_deck() {
    const EXPECTED_CARDS: [Card; 20] = [
        Card{suit: Suit::Hearts, rank: Rank::Unter},
        Card{suit: Suit::Hearts, rank: Rank::Ober},
        Card{suit: Suit::Hearts, rank: Rank::King},
        Card{suit: Suit::Hearts, rank: Rank::Ten},
        Card{suit: Suit::Hearts, rank: Rank::Ace},

        Card{suit: Suit::Bells, rank: Rank::Unter},
        Card{suit: Suit::Bells, rank: Rank::Ober},
        Card{suit: Suit::Bells, rank: Rank::King},
        Card{suit: Suit::Bells, rank: Rank::Ten},
        Card{suit: Suit::Bells, rank: Rank::Ace},
        
        Card{suit: Suit::Acorns, rank: Rank::Unter},
        Card{suit: Suit::Acorns, rank: Rank::Ober},
        Card{suit: Suit::Acorns, rank: Rank::King},
        Card{suit: Suit::Acorns, rank: Rank::Ten},
        Card{suit: Suit::Acorns, rank: Rank::Ace},
        
        Card{suit: Suit::Leaves, rank: Rank::Unter},
        Card{suit: Suit::Leaves, rank: Rank::Ober},
        Card{suit: Suit::Leaves, rank: Rank::King},
        Card{suit: Suit::Leaves, rank: Rank::Ten},
        Card{suit: Suit::Leaves, rank: Rank::Ace},
    ];

    let actual_cards = generate_deck();

    assert_eq!(EXPECTED_CARDS.len(), actual_cards.len());
    
    for card in EXPECTED_CARDS.iter() {
        assert!(actual_cards.contains(card));
    }
}

#[test]
fn test_first_beats_second_same_suit() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ace};
    let card2 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let trump = Suit::Bells;

    assert!(first_beats_second(card1, card2, trump));
    assert!(!first_beats_second(card2, card1, trump));
}

#[test]
fn test_first_beats_second_same_suit_trump() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ace};
    let card2 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let trump = Suit::Leaves;

    assert!(first_beats_second(card1, card2, trump));
    assert!(!first_beats_second(card2, card1, trump));
}

#[test]
fn test_first_beats_second_different_suit() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let card2 = Card {suit: Suit::Bells, rank: Rank::Ace};
    let trump = Suit::Hearts;

    assert!(first_beats_second(card1, card2, trump));
    assert!(first_beats_second(card2, card1, trump));
}


#[test]
fn test_first_beats_second_different_suit_trump() {
    let card1 = Card {suit: Suit::Leaves, rank: Rank::Ten};
    let card2 = Card {suit: Suit::Bells, rank: Rank::King};
    let trump = Suit::Bells;

    assert!(!first_beats_second(card1, card2, trump));
    assert!(first_beats_second(card2, card1, trump));
}
