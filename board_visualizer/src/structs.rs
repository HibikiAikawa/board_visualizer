use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardUnit{
    pub price: f32,
    pub size: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board{
    pub asks: Vec<BoardUnit>,  // 昇順
    pub bids: Vec<BoardUnit>,  // 降順
    pub broadcast_timestamp: String,  // 配信された時間
    pub processing_timestamp: String,  // このライブラリで処理までされた時間
}