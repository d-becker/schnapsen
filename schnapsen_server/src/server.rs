use schnapsen_core::schnapsen::{ErrorKind, Game, GameAdapter, PlayerId};

use schnapsen_core::message;
use schnapsen_core::message::{ServerMessage, FullPlayerGameState,
                              FullStateUpdate, Request, RequestData, Response};

pub trait Connection {
    fn get_message(&self) -> (PlayerId, Request);
    fn send_message(&self, player_id: PlayerId, message: ServerMessage);
}

pub struct Server<C: Connection> {
    state_number: u32,
    game_adapter: GameAdapter,
    connection: C
}

impl<C: Connection> Server<C> {
    pub fn start(&mut self) {
        loop {
            let (player_id, request) = self.connection.get_message();
            let (response, state1, state2)
                = self.handle_request(player_id, request);

            let response_message
                = ServerMessage {response: Some(response),
                                 state_update: state1};
            self.connection.send_message(player_id, response_message);

            if state2.is_some() {
                let message_to_other_player
                    = ServerMessage {response: None, state_update: state2};
                self.connection.send_message(player_id.other(),
                                             message_to_other_player);
            }
        }
    }
    
    fn handle_request(&mut self, player_id: PlayerId, request: Request)
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
