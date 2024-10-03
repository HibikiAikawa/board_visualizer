use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub jsonrpc: String,
    pub id: u32,
    pub op: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketOrderBookData {
    pub topic: String,
    pub r#type: String,
    pub ts: u64,
    pub data: BybitOrderbookData,
    pub cts: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketTradeData {
    pub topic: String,
    pub r#type: String,
    pub ts: u64,
    pub data: Vec<BybitTradeData>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BybitOrderbookData {
    pub s: String,
    pub b: Vec<[String; 2]>,
    pub a: Vec<[String; 2]>,
    pub u: u64,
    pub seq: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BybitTradeData {
    pub T: u64,
    pub s: String,
    pub S: String,
    pub v: String,
    pub p: String,
    pub L: String,
    pub i: String,
    pub BT: bool
}
    // pub mP: String,  for option
    // pub iP: String,  for option
    // pub mIv: String, for option
    // pub iv: String,  for option
