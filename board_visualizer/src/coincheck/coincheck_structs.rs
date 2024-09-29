use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub jsonrpc: String,
    pub id: u32,
    pub r#type: String,
    pub channel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoincheckBoardUnit{
    pub price: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoincheckBoard {
    pub bids: Vec<CoincheckBoardUnit>,
    pub asks: Vec<CoincheckBoardUnit>,
    pub last_update_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketBoardData{
    pub symbol: String,
    pub board: CoincheckBoard
}
