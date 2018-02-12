use cards::{Card, Suit};

pub enum RequestData {
    Close,
    ExchangeTrump,
    Twenty(Suit),
    Forty,
    Declare,
    Trick(Card)
}

pub struct Request {
    id: u32,
    data: RequestData
}

pub struct Response {
    request_id: u32,
    result: Result<(), String>
}
