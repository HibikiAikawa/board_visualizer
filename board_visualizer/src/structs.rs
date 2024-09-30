use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardUnit{
    pub price: f32,
    pub size: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Board{
    pub exchange: Exchange,
    pub pair: Pair,
    pub instrument: Instrument,
    pub asks: Vec<BoardUnit>,  // 昇順
    pub bids: Vec<BoardUnit>,  // 降順
    pub broadcast_timestamp: String,  // 配信された時間
    pub processing_timestamp: String,  // このライブラリで処理までされた時間
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "asks:")?;
        for ask in self.asks.iter().rev() {
            writeln!(f, "[{}, {}]", ask.price, ask.size)?;
        }
        writeln!(f, "=======================================")?;
        for bid in self.bids.iter() {
            writeln!(f, "[{}, {}]", bid.price, bid.size)?;
        }
        writeln!(f, "bids:")?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Exchange {
    Coincheck,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pair {
    BtcJpy
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Instrument {
    Spot
}