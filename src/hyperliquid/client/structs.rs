use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Msg<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub subscription: T,
}

#[derive(Serialize, Deserialize)]
pub struct SubscribeMessage {
    pub r#type: String,
    pub coin: String, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketData<T>{
    pub channel: String,
    pub data: T
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsBook {
    pub coin: String,
    pub levels: [Vec<WsLevel>; 2],
    pub time: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsLevel {
    pub px: String, // price
    pub sz: String, // size
    pub n: u64      // number of orders
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsTrade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub hash: String,
    pub time: u64,
    pub tid: u64 // ID unique across all assets
  }
  