use cards::{Card, Suit};
use schnapsen::player::PlayerId;

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicGameData {
    pub trump: Suit,
    pub closed: bool,
    pub winner: Option<PlayerId>,
    pub player_on_lead: PlayerId,
    pub first_card_in_trick: Option<Card>
}

impl Default for PublicGameData {
    fn default() -> PublicGameData {
        PublicGameData {
            trump: Suit::Hearts, closed: false,
            winner: None, player_on_lead: PlayerId::Player1,
            first_card_in_trick: None
        }
    }
}

impl PublicGameData {
    pub fn player_on_lead(&self) -> PlayerId {
        self.player_on_lead
    }

    pub fn player_on_turn(&self) -> PlayerId {
        match self.first_card_in_trick {
            Some(_) => self.player_on_lead.other(),
            None => self.player_on_lead
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }
}
