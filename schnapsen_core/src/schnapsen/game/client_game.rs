use schnapsen::{Card, Suit};
use schnapsen::game::Game;
use schnapsen::player::{DummyPlayer, IPlayer, Player, PlayerData, PlayerId};
use schnapsen::stock::DummyStock;

pub fn new_client_game(player_id: PlayerId,
                       winner: Option<PlayerId>,
                       trump: Suit,
                       first_card_in_trick: Option<Card>,
                       stock: DummyStock,
                       player_on_lead: PlayerId,
                       hand: Vec<Card>,
                       wins: Vec<Card>,
                       twenties: Vec<Suit>,
                       forty: Option<Suit>) -> Game<DummyStock> {
    let real_player : Box<IPlayer> = Box::new(Player::new(
        PlayerData {name: "".to_string(), hand, wins, twenties, forty}));
    let dummy_player : Box<IPlayer> = Box::new(DummyPlayer);

    let (player1, player2) = match player_id {
        PlayerId::Player1 => (real_player, dummy_player),
        PlayerId::Player2 => (dummy_player, real_player)
    };

    Game {winner,
          stock,
          trump,
          player1,
          player2,
          player_on_lead,
          first_card_in_trick}
}
