use cards::{Card, Rank, Suit};
use schnapsen;

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestData {
    Close,
    ExchangeTrump,
    PlayTwenty(Card),
    TwentyDeclareWin(Suit),
    PlayForty(Card),
    FortyDeclareWin,
    DeclareWin,
    PlayCard(Card)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: u32,
    pub data: RequestData
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub request_id: u32,
    pub result: Result<(), schnapsen::ErrorKind>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullPlayerGameState {
    pub winner: Option<schnapsen::PlayerId>,
    pub trump: Suit,
    pub first_card_in_trick: Option<Card>,
        
    pub stock_size: u32,
    pub trump_card_rank: Option<Rank>,
    pub stock_closed: bool,

    pub on_lead: schnapsen::PlayerId,
    pub hand: Vec<Card>,
    pub wins: Vec<Card>,
    pub twenties: Vec<Suit>,
    pub forty: Option<Suit>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullStateUpdate {
    pub state_number: u32,
    pub state: FullPlayerGameState
}

pub fn game_to_game_state<STOCK: schnapsen::IStock>(
    player_id: schnapsen::PlayerId,
    game: &schnapsen::Game<STOCK>)
    -> FullPlayerGameState
{    
    FullPlayerGameState {
        winner: game.winner(),
        trump: game.trump(),
        first_card_in_trick: game.get_first_card_in_trick(),

        stock_size: game.get_stock().len() as u32,
        trump_card_rank: game.get_stock().trump_card().map(|card| card.rank()),
        stock_closed: game.get_stock().is_closed(),

        on_lead: game.player_id_on_lead(),
        hand: game.get_player(player_id).get_hand().to_vec(),
        wins: game.get_player(player_id).get_wins().to_vec(),
        twenties: game.get_player(player_id).get_twenties().to_vec(),
        forty: *game.get_player(player_id).get_forty(),
    }
}

pub fn game_state_to_client_game(player_id: schnapsen::PlayerId,
                                 game_state: FullPlayerGameState)
                                 -> schnapsen::Game<schnapsen::DummyStock> {
    let trump_card = game_state.trump_card_rank.map(
        |rank| Card::new(game_state.trump, rank));
    let stock = schnapsen::DummyStock::new(game_state.stock_size as usize,
                                           trump_card,
                                           game_state.stock_closed);
    
    schnapsen::client_game::new_client_game(player_id,
                                            game_state.winner,
                                            game_state.trump,
                                            game_state.first_card_in_trick,
                                            stock,
                                            game_state.on_lead,
                                            game_state.hand,
                                            game_state.wins,
                                            game_state.twenties,
                                            game_state.forty)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json() {
        let id: u32 = 1;
        let request = Request {id, data: RequestData::TwentyDeclareWin(Suit::Leaves)};
        
        let serialized = ::serde_json::to_string_pretty(&request).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: Request = ::serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
}
