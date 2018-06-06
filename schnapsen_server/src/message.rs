use schnapsen_core::cards::{Card, Suit};
use schnapsen_core::schnapsen;

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestData {
    Close,
    ExchangeTrump,
    PlayTwenty(Card),
    TwentyDeclareWin(Suit),
    PlayForty(Card),
    FortyDeclareWin,
    Declare,
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
