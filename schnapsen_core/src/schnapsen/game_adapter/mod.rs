use super::{ErrorKind, Players};
use super::game::Game;

pub struct GameAdapter {
    game: Game
}

impl Default for GameAdapter {
    fn default() -> Self {
        GameAdapter::new(Game::default())
    }
}

impl GameAdapter {
    pub fn new(game: Game) -> GameAdapter {
        GameAdapter {game}
    }
    
    pub fn get_game_as_player(&mut self, player: Players)
                              -> Result<&mut Game, ErrorKind> {
        if self.game.player_on_turn() == player {
            Ok(&mut self.game)
        } else {
            Err(ErrorKind::NotPlayersTurn)
        }
    }
}

#[cfg(test)]
mod tests;
