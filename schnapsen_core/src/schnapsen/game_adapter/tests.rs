use super::*;

#[test]
fn test_get_game_player_not_on_turn() {
    let game = Game::default();
    let player_id_not_on_turn = game.player_id_on_turn().other();

    let mut adapter = GameAdapter::new(game);

    let result = adapter.get_game_as_player(player_id_not_on_turn);
    assert!(result.is_err());
    assert_eq!(ErrorKind::NotPlayersTurn, result.unwrap_err());
}

#[test]
fn test_get_game_ok() {
    let game = Game::default();
    let player_id_on_turn = game.player_id_on_turn();

    let mut adapter = GameAdapter::new(game);

    let result = adapter.get_game_as_player(player_id_on_turn);
    assert!(result.is_ok());
}
