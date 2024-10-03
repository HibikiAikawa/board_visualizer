use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use tokio::sync::mpsc;
use futures::SinkExt;


mod structs;
use structs::{Msg, WebsocketBoardData, CoincheckBoard};

use super::super::structs::{BoardUnit, Board, Exchange, Pair, Instrument};

const WEBSOCKET_ROOT_URL: &str = "wss://ws-api.coincheck.com/";

pub async fn run(max_board_size: usize) { 
    let (ws_stream, _) = connect_async(WEBSOCKET_ROOT_URL)
        .await
        .expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let msg = Msg {
        jsonrpc: "2.0".to_string(),
        id: 1,
        r#type: "subscribe".to_string(),
        channel: "btc_jpy-orderbook".to_string(),
    };
    let msg_str = serde_json::to_string(&msg).unwrap();
    write.send(Message::Text(msg_str)).await.unwrap();
    
    // 板情報
    let mut asks: Vec<BoardUnit> = Vec::new();
    let mut bids: Vec<BoardUnit> = Vec::new();

    // 保存用ベクター
    let mut board_vec: Vec<Board> = Vec::new();
    let save_length = 10000;

    while let Some(message) = read.next().await {
        match message.unwrap() {
            Message::Close(_) => break,
            Message::Ping(ping) => write.send(Message::Pong(ping)).await.unwrap(),
            Message::Text(text) => {
                let data = serde_json::from_str::<WebsocketBoardData>(&text).unwrap();
                // 差分の板情報を前処理
                let (asks_diff, bids_diff) = preprocessing(&data.board);
                // 板情報をアップデート
                update(&mut asks, &asks_diff, max_board_size, true);
                update(&mut bids, &bids_diff, max_board_size, false);

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
                // 板情報を構造体に変換
                let board: Board = Board{
                    exchange: Exchange::Coincheck,
                    pair: Pair::BtcJpy,
                    instrument: Instrument::Spot,
                    asks: asks_truncated,
                    bids: bids_truncated,
                    broadcast_timestamp: data.board.last_update_at,
                    processing_timestamp: chrono::Local::now().to_rfc3339(),
                };

                // println!("{:?}", board);
                // 板情報を保存
                board_vec.push(board);
                println!("length: {}", board_vec.len());

                if board_vec.len() > save_length - 1 {
                    // Boardをjson形式で保存
                    let board_json = serde_json::to_string(&board_vec).unwrap();
                    std::fs::write("../data/sample/coincheck_board.json", board_json).unwrap();
                    break;
                }
                
            }, 
            _ => (),
        }
    }
}


// コインチェックの板情報を前処理して板情報差分に変換
// ソートはしない
fn preprocessing(raw_board_diff: &CoincheckBoard) -> (Vec<BoardUnit>, Vec<BoardUnit>){
    let asks_diff: Vec<BoardUnit> = raw_board_diff.asks.iter()
                                                       .map(|x| BoardUnit{price: x.price.parse().unwrap(), size: x.size.parse().unwrap()})
                                                       .collect();
    let bids_diff: Vec<BoardUnit> = raw_board_diff.bids.iter()
                                                       .map(|x| BoardUnit{price: x.price.parse().unwrap(), size: x.size.parse().unwrap()})
                                                       .collect();
    (asks_diff, bids_diff)
}

fn update(board: &mut Vec<BoardUnit>, board_diff: &Vec<BoardUnit>, max_board_size: usize, ascending: bool) {
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

// fn create_new_board(board: &Board) -> (Vec<[f32; 2]>, Vec<[f32; 2]>) {
//     let mut  board_bid: Vec<[f32;2]> = Vec::new();
//     let mut  board_ask: Vec<[f32;2]> = Vec::new();
//     for board_unit in &board.bids {board_bid.push([board_unit.price, board_unit.size]);}
//     for board_unit in &board.asks {board_ask.push([board_unit.price, board_unit.size]);}
//     board_ask.sort_by(|a: &[f32; 2], b| a[0].partial_cmp(&b[0]).unwrap()); // 昇順
//     board_bid.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap()); // 降順
//     (board_ask, board_bid)
// }

#[tokio::main]
async fn main() {
    // let (tx, mut rx) = mpsc::channel(100);
    run(20).await;
    // test();
}