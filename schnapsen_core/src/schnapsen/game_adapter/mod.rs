use schnapsen::ErrorKind;
use schnapsen::game::Game;
use schnapsen::player::PlayerId;
use schnapsen::stock::Stock;

pub struct GameAdapter {
    game: Game<Stock>
}

impl Default for GameAdapter {
    fn default() -> Self {
        GameAdapter::new(Game::default())
    }
}

impl GameAdapter {
    pub fn new(game: Game<Stock>) -> GameAdapter {
        GameAdapter {game}
    }
    
    pub fn get_game_as_player(&mut self, player: PlayerId)
                              -> Result<&mut Game<Stock>, ErrorKind> {
        if self.game.player_id_on_turn() == player {
            Ok(&mut self.game)
        } else {
            Err(ErrorKind::NotPlayersTurn)
        }
    }

    pub fn get_game_view(&self) -> &Game<Stock> {
        &self.game
    }
}

#[cfg(test)]
mod tests;
