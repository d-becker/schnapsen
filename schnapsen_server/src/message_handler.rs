use schnapsen_core::schnapsen::{Game, GameAdapter, PlayerId};

use schnapsen_core::message::{FullStateUpdate, Request, Response};

pub struct MessageHandler {
    state_number: u32,
    game_adapter: GameAdapter
}

impl MessageHandler {
    fn handle_request(player_id: PlayerId, request: Request)
                      -> (Response,
                          Option<FullStateUpdate>,
                          Option<FullStateUpdate>) {
        unimplemented!();
    }
}
