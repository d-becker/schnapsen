use super::*;

#[test]
fn test_value_unter() {
    let card = Card::new(Suite::Hearts, Rank::Unter);
    assert_eq!(2, value(card));
}

#[test]
fn test_value_ober() {
    let card = Card::new(Suite::Bells, Rank::Ober);
    assert_eq!(3, value(card));
}

#[test]
fn test_value_king() {
    let card = Card::new(Suite::Leaves, Rank::King);
    assert_eq!(4, value(card));
}

#[test]
fn test_value_ten() {
    let card = Card::new(Suite::Hearts, Rank::Ten);
    assert_eq!(10, value(card));
}

#[test]
fn test_value_ace() {
    let card = Card::new(Suite::Acorns, Rank::Ace);
    assert_eq!(11, value(card));
}

#[test]
fn test_value_illegal_card() {
    let card = Card::new(Suite::Acorns, Rank::Seven);
    assert_eq!(0, value(card));
}

#[test]
fn test_generate_deck() {
    const EXPECTED_CARDS: [Card; 20] = [
        Card{suite: Suite::Hearts, rank: Rank::Unter},
        Card{suite: Suite::Hearts, rank: Rank::Ober},
        Card{suite: Suite::Hearts, rank: Rank::King},
        Card{suite: Suite::Hearts, rank: Rank::Ten},
        Card{suite: Suite::Hearts, rank: Rank::Ace},

        Card{suite: Suite::Bells, rank: Rank::Unter},
        Card{suite: Suite::Bells, rank: Rank::Ober},
        Card{suite: Suite::Bells, rank: Rank::King},
        Card{suite: Suite::Bells, rank: Rank::Ten},
        Card{suite: Suite::Bells, rank: Rank::Ace},
        
        Card{suite: Suite::Acorns, rank: Rank::Unter},
        Card{suite: Suite::Acorns, rank: Rank::Ober},
        Card{suite: Suite::Acorns, rank: Rank::King},
        Card{suite: Suite::Acorns, rank: Rank::Ten},
        Card{suite: Suite::Acorns, rank: Rank::Ace},
        
        Card{suite: Suite::Leaves, rank: Rank::Unter},
        Card{suite: Suite::Leaves, rank: Rank::Ober},
        Card{suite: Suite::Leaves, rank: Rank::King},
        Card{suite: Suite::Leaves, rank: Rank::Ten},
        Card{suite: Suite::Leaves, rank: Rank::Ace},
    ];

    let actual_cards = generate_deck();

    assert_eq!(EXPECTED_CARDS.len(), actual_cards.len());
    
    for card in EXPECTED_CARDS.iter() {
        assert!(actual_cards.contains(card));
    }
}

#[test]
fn test_first_beats_second_same_suite() {
    let card1 = Card {suite: Suite::Leaves, rank: Rank::Ace};
    let card2 = Card {suite: Suite::Leaves, rank: Rank::Ten};
    let trumph = Suite::Bells;

    assert!(first_beats_second(card1, card2, trumph));
    assert!(!first_beats_second(card2, card1, trumph));
}

#[test]
fn test_first_beats_second_same_suite_trumph() {
    let card1 = Card {suite: Suite::Leaves, rank: Rank::Ace};
    let card2 = Card {suite: Suite::Leaves, rank: Rank::Ten};
    let trumph = Suite::Leaves;

    assert!(first_beats_second(card1, card2, trumph));
    assert!(!first_beats_second(card2, card1, trumph));
}

#[test]
fn test_first_beats_second_different_suite() {
    let card1 = Card {suite: Suite::Leaves, rank: Rank::Ten};
    let card2 = Card {suite: Suite::Bells, rank: Rank::Ace};
    let trumph = Suite::Hearts;

    assert!(first_beats_second(card1, card2, trumph));
    assert!(first_beats_second(card2, card1, trumph));
}


#[test]
fn test_first_beats_second_different_suite_trumph() {
    let card1 = Card {suite: Suite::Leaves, rank: Rank::Ten};
    let card2 = Card {suite: Suite::Bells, rank: Rank::King};
    let trumph = Suite::Bells;

    assert!(!first_beats_second(card1, card2, trumph));
    assert!(first_beats_second(card2, card1, trumph));
}

#[test]
fn test_new_game() {
    let game = Game::new();

    assert_eq!(10, game.deck.len());
    assert_eq!(5, game.player1.hand.len());
    assert_eq!(5, game.player2.hand.len());

    assert_eq!(game.deck[0].suite(), game.trumph);
}

#[test]
fn test_trumph_card_available() {
    let game = Game::new();

    assert_eq!(Some(game.deck[0]), game.trumph_card());
}

#[test]
fn test_trumph_card_folded() {
    let mut game = Game::new();
    game.folded = true;

    assert_eq!(None, game.trumph_card());
}

#[test]
fn test_trumph_card_no_card_left() {
    let game = Game {
        deck: Vec::new(),
        trumph: Suite::Leaves,
        folded: false,
        player1: Player::new("Player1".to_string()),
        player2: Player::new("Player2".to_string()),
        player1_next: true
    };

    assert_eq!(None, game.trumph_card());
}

#[test]
fn test_fold_ok() {
    let mut game = Game::new();

    assert!(!game.is_folded());

    let result = game.fold();

    assert!(result);
    assert!(game.is_folded());
}

#[test]
fn test_fold_not_enough_cards_left() {
    let mut game = Game {
        deck: vec![Card::new(Suite::Leaves, Rank::Ace),
                   Card::new(Suite::Leaves, Rank::Ten)],
        trumph: Suite::Leaves,
        folded: false,
        player1: Player::new("Player1".to_string()),
        player2: Player::new("Player2".to_string()),
        player1_next: true
    };

    assert!(!game.is_folded());

    let result = game.fold();

    assert!(!result);
    assert!(!game.is_folded());
}

#[test]
fn test_next_turn_invalid_player1_index() {
    let mut game = Game::new();
    game.player1.hand.truncate(4);

    let result = game.next_turn(4, 0);
    assert!(!result);
}

#[test]
fn test_next_turn_invalid_player2_index() {
    let mut game = Game::new();
    game.player2.hand.truncate(4);

    let result = game.next_turn(0, 4);
    assert!(!result);
}

#[test]
fn test_next_turn_folded_illegal_suite() {
    let deck = vec![Card{suite: Suite::Hearts, rank: Rank::Unter},
                    Card{suite: Suite::Hearts, rank: Rank::Ober},
                    Card{suite: Suite::Hearts, rank: Rank::King},
                    Card{suite: Suite::Hearts, rank: Rank::Ten},
                    Card{suite: Suite::Hearts, rank: Rank::Ace},
                    Card{suite: Suite::Acorns, rank: Rank::Unter}];

    let trumph = deck[0].suite();

    let player1_hand = vec![Card{suite: Suite::Acorns, rank: Rank::Ten},
                            Card{suite: Suite::Acorns, rank: Rank::Ace},
                            Card{suite: Suite::Leaves, rank: Rank::Unter},
                            Card{suite: Suite::Leaves, rank: Rank::Ober},
                            Card{suite: Suite::Leaves, rank: Rank::King}];

    let player2_hand = vec![Card{suite: Suite::Acorns, rank: Rank::Ober},
                            Card{suite: Suite::Acorns, rank: Rank::King},
                            Card{suite: Suite::Leaves, rank: Rank::Ten},
                            Card{suite: Suite::Leaves, rank: Rank::Ace},
                            Card{suite: Suite::Bells, rank: Rank::Unter}];

    let player1 = Player {name: "Player1".to_string(), hand: player1_hand, wins: Vec::new()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand, wins: Vec::new()};

    let mut game = Game {
        deck, trumph, folded: false, player1, player2, player1_next: true
    };

    let folding_successful = game.fold();

    assert!(folding_successful);
    assert!(game.is_folded());

    let result = game.next_turn(0, 2);
    assert!(!result);
}

#[test]
fn test_next_turn_folded_must_be_higher() {
    let deck = Vec::new();

    let trumph = Suite::Hearts;

    let player1_hand = vec![Card{suite: Suite::Acorns, rank: Rank::Ten},
                            Card{suite: Suite::Acorns, rank: Rank::King},
                            Card{suite: Suite::Leaves, rank: Rank::Unter},
                            Card{suite: Suite::Leaves, rank: Rank::Ober},
                            Card{suite: Suite::Leaves, rank: Rank::King}];

    let player2_hand = vec![Card{suite: Suite::Acorns, rank: Rank::Ober},
                            Card{suite: Suite::Acorns, rank: Rank::Ace},
                            Card{suite: Suite::Leaves, rank: Rank::Ten},
                            Card{suite: Suite::Leaves, rank: Rank::Ace},
                            Card{suite: Suite::Bells, rank: Rank::Unter}];

    let player1 = Player {name: "Player1".to_string(), hand: player1_hand, wins: Vec::new()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand, wins: Vec::new()};

    let mut game = Game {
        deck, trumph, folded: false, player1, player2, player1_next: true
    };

    let result = game.next_turn(0, 0);
    assert!(!result);
}

#[test]
fn test_next_turn_folded_must_use_trumph() {
    let deck = Vec::new();

    let trumph = Suite::Hearts;

    let player1_hand = vec![Card{suite: Suite::Acorns, rank: Rank::Ten},
                            Card{suite: Suite::Acorns, rank: Rank::King},
                            Card{suite: Suite::Leaves, rank: Rank::Unter},
                            Card{suite: Suite::Leaves, rank: Rank::Ober},
                            Card{suite: Suite::Leaves, rank: Rank::King}];

    let player2_hand = vec![Card{suite: Suite::Bells, rank: Rank::Ober},
                            Card{suite: Suite::Bells, rank: Rank::Ace},
                            Card{suite: Suite::Leaves, rank: Rank::Ten},
                            Card{suite: Suite::Leaves, rank: Rank::Ace},
                            Card{suite: Suite::Hearts, rank: Rank::Unter}];

    let player1 = Player {name: "Player1".to_string(), hand: player1_hand, wins: Vec::new()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand, wins: Vec::new()};

    let mut game = Game {
        deck, trumph, folded: false, player1, player2, player1_next: true
    };

    let result = game.next_turn(0, 0);
    assert!(!result);
}

#[test]
fn test_next_turn_ok() {
    let deck = vec![
        Card{suite: Suite::Hearts, rank: Rank::Unter},
        Card{suite: Suite::Hearts, rank: Rank::Ober},
        Card{suite: Suite::Hearts, rank: Rank::King},
        Card{suite: Suite::Hearts, rank: Rank::Ten},
        Card{suite: Suite::Hearts, rank: Rank::Ace},
        
        Card{suite: Suite::Acorns, rank: Rank::Unter},
        Card{suite: Suite::Acorns, rank: Rank::Ober},
        Card{suite: Suite::Acorns, rank: Rank::King},
        Card{suite: Suite::Acorns, rank: Rank::Ten},
        Card{suite: Suite::Acorns, rank: Rank::Ace},
    ];

    let trumph = deck[0].suite();

    let player1_hand = vec![
        Card{suite: Suite::Bells, rank: Rank::Unter},
        Card{suite: Suite::Bells, rank: Rank::Ober},
        Card{suite: Suite::Bells, rank: Rank::King},
        Card{suite: Suite::Leaves, rank: Rank::Ace},
        Card{suite: Suite::Bells, rank: Rank::Ten},
    ];

    let player2_hand = vec![
        Card{suite: Suite::Leaves, rank: Rank::Unter},
        Card{suite: Suite::Leaves, rank: Rank::Ober},
        Card{suite: Suite::Leaves, rank: Rank::King},
        Card{suite: Suite::Leaves, rank: Rank::Ten},
        Card{suite: Suite::Bells, rank: Rank::Ace},
    ];

    let player1 = Player {name: "Player1".to_string(), hand: player1_hand, wins: Vec::new()};
    let player2 = Player {name: "Player2".to_string(), hand: player2_hand, wins: Vec::new()};
    
    let mut game = Game {
        deck, trumph, folded: false, player1, player2, player1_next: true
    };

    let card1 = game.player1.hand[4];
    let card2 = game.player2.hand[4];
    let result = game.next_turn(4, 4);
    assert!(result);

    assert!(!game.player1.hand.contains(&card1));
    assert!(!game.player2.hand.contains(&card2));

    assert_eq!(5, game.player1.hand.len());
    assert_eq!(5, game.player2.hand.len());

    assert_eq!(8, game.deck.len());
    
    assert!(game.player2.wins.contains(&card1));
    assert!(game.player2.wins.contains(&card2));
    assert_eq!(game.player2.name, game.next_player_name());
}
