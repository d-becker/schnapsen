use super::*;
use schnapsen::IStock;

#[test]
fn test_json() {
    let id: u32 = 1;
    let request = Request {id, data: RequestData::TwentyDeclareWin(Suit::Leaves)};
    
    let serialized = ::serde_json::to_string_pretty(&request).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Request = ::serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

#[test]
fn test_game_state_conversions() {
    let game_state = FullPlayerGameState {
        winner: Some(schnapsen::PlayerId::Player1),
        trump: Suit::Leaves,
        first_card_in_trick: Some(Card::new(Suit::Leaves, Rank::King)),

        stock_size: 4,
        trump_card_rank: Some(Rank::Ten),
        stock_closed: false,

        on_lead: schnapsen::PlayerId::Player1,
        hand: vec![Card::new(Suit::Leaves, Rank::Ober),
                   Card::new(Suit::Bells, Rank::King),
                   Card::new(Suit::Bells, Rank::Ten),
                   Card::new(Suit::Bells, Rank::Ace)],
        wins: vec![],
        twenties: vec![],
        forty: None
    };
    
    let player_id = schnapsen::PlayerId::Player1;
    let game = game_state_to_client_game(player_id,
                                         game_state.clone());

    assert_eq!(game_state.winner, game.winner());
    assert_eq!(game_state.trump, game.trump());
    assert_eq!(game_state.first_card_in_trick, game.get_first_card_in_trick());
    assert_eq!(game_state.stock_size, game.get_stock().len() as u32);
    assert_eq!(game_state.trump_card_rank,
               game.trump_card().map(|card| card.rank()));
    assert_eq!(game_state.stock_closed, game.is_closed());
    assert_eq!(game_state.on_lead, game.player_id_on_lead());
    assert_eq!(game_state.hand, game.get_player(game_state.on_lead).get_hand());
    assert_eq!(game_state.wins, game.get_player(game_state.on_lead).get_wins());
    assert_eq!(game_state.forty,
               *game.get_player(game_state.on_lead).get_forty());

    let game_state_converted_back = game_to_game_state(player_id, &game);
    assert_eq!(game_state, game_state_converted_back);
}
