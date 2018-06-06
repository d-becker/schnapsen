use super::*;
use cards::Rank;

#[test]
fn test_player_score() {
    let wins = vec![Card {suit: Suit::Leaves, rank: Rank::Ten},
                    Card {suit: Suit::Bells, rank: Rank::Ace},
                    Card {suit: Suit::Bells, rank: Rank::King},
                    Card {suit: Suit::Bells, rank: Rank::Ober}];

    let data = PlayerData {name: "Player".to_string(), wins,
                           twenties: vec![Suit::Bells, Suit::Hearts],
                           forty: Some(Suit::Leaves),
                           ..Default::default()};
    let player = Player::new(data);

    assert_eq!(28 + 2 * 20 + 40, player.score());
}
