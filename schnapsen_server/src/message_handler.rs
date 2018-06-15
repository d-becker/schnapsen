use schnapsen_core::schnapsen::{ErrorKind, Game, GameAdapter, PlayerId};

use schnapsen_core::message;
use schnapsen_core::message::{FullPlayerGameState, FullStateUpdate,
                              Request, RequestData, Response};

pub struct MessageHandler {
    state_number: u32,
    game_adapter: GameAdapter
}

impl MessageHandler {
    pub fn handle_request(&mut self, player_id: PlayerId, request: Request)
                          -> (Response,
                              Option<FullStateUpdate>,
                              Option<FullStateUpdate>)
    {
        let response_res = self.response_result(player_id, &request);
        let response = Response {request_id: request.id, result: response_res};

        if response_res.is_ok() {
            self.state_number += 1;

            let player1_state = message::game_to_game_state(
                PlayerId::Player1,
                self.game_adapter.get_game_view());
            let player2_state = message::game_to_game_state(
                PlayerId::Player2,
                self.game_adapter.get_game_view());

            let full_state1 = FullStateUpdate {
                state_number: self.state_number, state: player1_state};
            let full_state2 = FullStateUpdate {
                state_number: self.state_number, state: player2_state};
            
            (response, Some(full_state1), Some(full_state2))
        } else {
            (response, None, None)
        }
    }

    fn response_result(&mut self, player_id: PlayerId, request: &Request)
                       -> Result<(), ErrorKind>
    {
        let game = self.game_adapter.get_game_as_player(player_id)?;

        let result = match request.data {
            RequestData::Close => game.close(),
            RequestData::ExchangeTrump => game.exchange_trump(),
            RequestData::PlayTwenty(card) => game.play_card_twenty(card),
            RequestData::TwentyDeclareWin(suit)
                => game.declare_twenty_win(suit),
            RequestData::PlayForty(card) => game.play_card_forty(card),
            RequestData::FortyDeclareWin => game.declare_forty_win(),
            RequestData::DeclareWin => game.declare_win(),
            RequestData::PlayCard(card) => game.play_card(card)
                .map(|_| ()) // Discarding the dealt cards for now.
        };

        result
    }
}
