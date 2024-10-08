use std::fs::File;
use std::io::Result;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use tokio::sync::mpsc;
use futures::SinkExt;


mod structs;

use super::super::structs::{BoardUnit, Board, Exchange, Pair, Instrument};


// SPOT, PERP, OPTION, INVERSEでURLが違う
const WEBSOCKET_BASE_URL: &str = "wss://stream.bybit.com/v5/public";

pub async fn fetch(max_board_size: usize, save_time_min: i64, symbol: String, instrument: String, dir_path: String) { 
    println!("fetching bybit data...");
    println!("symbol: {}, instrument: {}", symbol, instrument);
    let url = if instrument == "spot" {
        "/spot"
    } else if instrument == "perp" {
        "/linear"
    } else {
        panic!("instrument must be spot, perp or futures");
    };

    let websocket_root_url = WEBSOCKET_BASE_URL.to_string() + url;
    let (ws_stream, _) = connect_async(websocket_root_url)
        .await
        .expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // SUBSCRIBE
    let msg = structs::Msg {
        jsonrpc: "2.0".to_string(),
        id: 1,
        op: "subscribe".to_string(),
        args: vec![
            "orderbook.200.".to_string() + &symbol,  // データ取得間隔を指定することができるとりあえず100msで取る
            "publicTrade.".to_string() + &symbol,
            ]
    };
    let msg_str = serde_json::to_string(&msg).unwrap();
    write.send(Message::Text(msg_str)).await.unwrap();
    
    // 板情報
    let mut asks: Vec<BoardUnit> = Vec::new();
    let mut bids: Vec<BoardUnit> = Vec::new();

    // 保存用ベクター
    let mut board_vec: Vec<Board> = Vec::new();
    let mut trade_vec: Vec<structs::BybitTradeData> = Vec::new();

    let start_time = chrono::Local::now();

    while let Some(message) = read.next().await {
        match message.unwrap() {
            Message::Close(_) => break,
            Message::Ping(ping) => write.send(Message::Pong(ping)).await.unwrap(),
            Message::Text(text) => {
                let board = serde_json::from_str::<structs::WebsocketOrderBookData>(&text);
                let trade = serde_json::from_str::<structs::WebsocketTradeData>(&text);
                match board {
                    Ok(data) => {
                        let _type =  data.r#type;
                        let bids_diff_raw = data.data.b;
                        let asks_diff_raw = data.data.a;

                        // 前処理
                        let bids_diff = preprocessing(&bids_diff_raw);
                        let asks_diff = preprocessing(&asks_diff_raw);
                        
                        // delta: 差分, snapshot: 昇順降順もソートされている
                        if _type == "delta" {
                            update(&mut asks, &asks_diff, true);
                            update(&mut bids, &bids_diff, false);
                        } else {  // snapshot ちゃんと機能しているか調べていない調べてない
                            asks = asks_diff;
                            bids = bids_diff;
                        }

                        // 板情報をmax_board_sizeに切り詰め
                        let asks_truncated: Vec<BoardUnit> = if asks.len() > max_board_size {
                            let mut asks_clone = asks.clone();
                            asks_clone.truncate(max_board_size);
                            asks_clone
                        } else {
                            asks.clone()
                        };
                        let bids_truncated: Vec<BoardUnit> = if bids.len() > max_board_size {
                            let mut bids_clone = bids.clone();
                            bids_clone.truncate(max_board_size);
                            bids_clone
                        } else {
                            bids.clone()
                        };

                        // Board構造体に変換
                        let board: Board = Board{
                            exchange: Exchange::Bybit,
                            pair: Pair::BtcUsdt,
                            instrument: Instrument::Spot,
                            asks: asks_truncated,
                            bids: bids_truncated,
                            broadcast_timestamp: data.ts.to_string(),
                            processing_timestamp: chrono::Local::now().to_rfc3339(),
                        };

                        // 板情報を保存
                        board_vec.push(board);
                        
                        // save_time_min分以上経過していたら保存
                        if (chrono::Local::now() - start_time).num_minutes() > save_time_min {
                            println!("saving data...");
                            // path名
                            // TODO /をdir_pathに含めるとエラーになるので注意.　いつか対策する
                            let board_path = dir_path.clone() + "/board_" + &symbol + "_" + &instrument + ".json";
                            let trade_path = dir_path.clone() + "/trade_" + &symbol + "_" + &instrument + ".json";
                            // Boardをjson形式で保存
                            let board_json = serde_json::to_string(&board_vec).unwrap();
                            std::fs::write(board_path, board_json).unwrap();
                            // Tradeをjson形式で保存
                            save_trade(trade_vec, trade_path.to_string()).unwrap();
                            break;
                        }
                    }
                    Err(_) => {}
                }

                match trade {
                    Ok(data) => {
                        let trades = data.data;
                        // 取引情報を保存
                        trade_vec.extend(trades);
                    },
                    Err(_) => {}
                }

                
            }, 
            _ => (),
        }
    }
}

fn preprocessing(board_diff_raw: &Vec<[String; 2]>) -> Vec<BoardUnit> {
    let mut board_diff: Vec<BoardUnit> = Vec::new();
    for board_unit in board_diff_raw {
        let price = board_unit[0].parse::<f32>().unwrap();
        let size = board_unit[1].parse::<f32>().unwrap();
        let new_board_unit = BoardUnit{price, size};
        board_diff.push(new_board_unit);
    }
    board_diff
}

fn update(board: &mut Vec<BoardUnit>, board_diff: &Vec<BoardUnit>, ascending: bool) {
    // Boardをfor文ですでにある場合には更新、ない場合には追加
    for board_unit in board_diff {
        let price = board_unit.price;
        let size = board_unit.size;
        let mut is_exist = false;
        for board_unit in board.iter_mut() {
            if board_unit.price == price {
                board_unit.size = size;
                is_exist = true;
                break;
            }
        }
        let new_board_unit = BoardUnit{price: price, size: size};
        if is_exist == false { board.push(new_board_unit); }
    }
    // size = 0の場合には削除
    board.retain(|x| x.size != 0.0);
    // 並び替える
    if ascending { 
        board.sort_by(|a: &BoardUnit, b: &BoardUnit| a.price.partial_cmp(&b.price).unwrap()); // 昇順
    } else {
        board.sort_by(|a: &BoardUnit, b: &BoardUnit| b.price.partial_cmp(&a.price).unwrap()); //　降順
    }
}

fn save_trade(trades: Vec<structs::BybitTradeData>, path: String) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer(file, &trades)?;
    Ok(())
}


#[tokio::main]
async fn main() {
    // let (tx, mut rx) = mpsc::channel(100);
    // run(20).await;
    // test();
}