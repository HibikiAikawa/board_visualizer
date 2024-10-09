use std::fs::File;
use std::io::Result;


use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use tokio::sync::mpsc;
use futures::SinkExt;

mod structs;

use super::super::structs::{BoardUnit, Board, Exchange, Pair, Instrument};

const WEBSOCKET_ROOT_URL: &str = "wss://api.hyperliquid.xyz/ws";

pub async fn run(max_board_size: usize) { 
    let (ws_stream, _) = connect_async(WEBSOCKET_ROOT_URL)
        .await
        .expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // ORDER_BOOK
    let wsbook = structs::SubscribeMessage {
        r#type: "l2Book".to_string(),
        coin: "PURR/USDC".to_string(),
    };

    let msg = structs::Msg::<structs::SubscribeMessage> {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "subscribe".to_string(),
        subscription: wsbook,
    };
    let msg_str = serde_json::to_string(&msg).unwrap();
    write.send(Message::Text(msg_str)).await.unwrap();
    
    // TRADE
    let wsbook = structs::SubscribeMessage {
        r#type: "trades".to_string(),
        coin: "PURR/USDC".to_string(),
    };

    let msg = structs::Msg {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "subscribe".to_string(),
        subscription: wsbook,
    };
    let msg_str = serde_json::to_string(&msg).unwrap();
    write.send(Message::Text(msg_str)).await.unwrap();

    // 板情報
    // MEMO データにはnumber of order項目もあったけど一旦無視
    let mut asks: Vec<BoardUnit> = Vec::new();
    let mut bids: Vec<BoardUnit> = Vec::new();

    // 保存用ベクター
    let mut board_vec: Vec<Board> = Vec::new();
    let mut trade_vec: Vec<structs::WsTrade> = Vec::new();
    let save_length = 10000;

    while let Some(message) = read.next().await {
        match message.unwrap() {
            Message::Close(_) => break,
            Message::Ping(ping) => write.send(Message::Pong(ping)).await.unwrap(),
            Message::Text(text) => {
                let wsbook = serde_json::from_str::<structs::WebsocketData<structs::WsBook>>(&text);
                let wstrade = serde_json::from_str::<structs::WebsocketData<Vec<structs::WsTrade>>>(&text);
                match wsbook {
                    Ok(data) => {
                        let board = data.data.levels;
                        let [bids, asks] = board;

                        // preprocessing
                        let mut asks: Vec<BoardUnit> = preprocessing(asks);
                        let mut bids: Vec<BoardUnit> = preprocessing(bids);

                        // 板情報をmax_board_sizeに切り詰め
                        if asks.len() > max_board_size { asks.truncate(max_board_size); }
                        if bids.len() > max_board_size { bids.truncate(max_board_size); }

                        // 板情報を構造体に変換
                        let board: Board = Board{
                            exchange: Exchange::Hyperliquid,
                            pair: Pair::PurrUsdc,
                            instrument: Instrument::Spot,
                            asks: asks,
                            bids: bids,
                            broadcast_timestamp: data.data.time.to_string(),
                            processing_timestamp: chrono::Local::now().to_rfc3339(),
                        };

                        // 板情報を保存
                        board_vec.push(board);
                        println!("length: {}", board_vec.len());

                        if board_vec.len() > save_length - 1 {
                            // Boardをjson形式で保存
                            let board_json = serde_json::to_string(&board_vec).unwrap();
                            std::fs::write("../data/sample/hyperliquid_board.json", board_json).unwrap();
                            // Tradeをjson形式で保存
                            save_trade(trade_vec, "../data/sample/hyperliquid_trade.json".to_string()).unwrap();
                            break;
                        }
                    },
                    Err(_) => (),
                }
                match wstrade {
                    Ok(data) => {
                        let trades = data.data;
                        // 取引情報を保存
                        trade_vec.extend(trades);
                    },
                    Err(_) => (),
                }
            }, 
            _ => (),
        }
    }
}


// コインチェックの板情報を前処理して板情報差分に変換
// ソートはしない
fn preprocessing(board: Vec<structs::WsLevel>) -> Vec<BoardUnit>{
    let board: Vec<BoardUnit> = board.iter()
                                         .map(|x| BoardUnit{price: x.px.parse().unwrap(), size: x.sz.parse().unwrap()})
                                         .collect();
    board
}

fn save_trade(trades: Vec<structs::WsTrade>, path: String) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer(file, &trades)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // let (tx, mut rx) = mpsc::channel(100);
    run(20).await;
    // test();
}