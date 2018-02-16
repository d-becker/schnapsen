use cards::{Card, Suit};
use schnapsen;

pub enum RequestData {
    Close,
    ExchangeTrump,
    Twenty(Suit),
    Forty,
    Declare,
    Trick(Card)
}

pub struct Request {
    pub id: u32,
    pub data: RequestData
}

pub struct Response {
    pub request_id: u32,
    pub result: Result<(), schnapsen::ErrorKind>
}
