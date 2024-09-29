use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use tokio::sync::mpsc;
use futures::SinkExt;

use crate::coincheck::coincheck_structs::{Msg, WebsocketBoardData, CoincheckBoard};

use structs::structs::BoardUnit;

const WEBSOCKET_ROOT_URL: &str = "wss://ws-api.coincheck.com/";

pub async fn run() { 
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
    
    let mut asks: Vec<BoardUnit> = Vec::new();
    let mut bids: Vec<BoardUnit> = Vec::new();
    while let Some(message) = read.next().await {
        match message.unwrap() {
            Message::Close(_) => break,
            Message::Ping(ping) => write.send(Message::Pong(ping)).await.unwrap(),
            Message::Text(text) => {
                let data = serde_json::from_str::<WebsocketBoardData>(&text).unwrap();
                // 差分の板情報を前処理
                let (asks_diff, bids_diff) = preprocessing(data.board);
                // 板情報をアップデート
                // asks, bids = update(&mut asks, &asks_diff);
                // 板情報を構造体に変換
                // board = create_new_board(asks, bids);

                // boardをyield
                println!("{:?}", asks_diff);
            }, 
            _ => (),
        }
    }
}


// コインチェックの板情報を前処理して板情報差分に変換
// ソートはしない
fn preprocessing(raw_board_diff: CoincheckBoard) -> (Vec<BoardUnit>, Vec<BoardUnit>){
    let asks_diff: Vec<BoardUnit> = raw_board_diff.asks.iter()
                                                       .map(|x| BoardUnit{price: x.price.parse().unwrap(), size: x.size.parse().unwrap()})
                                                       .collect();
    let bids_diff: Vec<BoardUnit> = raw_board_diff.bids.iter()
                                                       .map(|x| BoardUnit{price: x.price.parse().unwrap(), size: x.size.parse().unwrap()})
                                                       .collect();
    (asks_diff, bids_diff)
}

fn update(board: &mut Vec<[f32; 2]>, board_diff: &Vec<BoardUnit>) {
    // Boardをfor文ですでにある場合には更新、ない場合には追加
    for board_unit in board_diff {
        let price = board_unit.price;
        let size = board_unit.size;
        let mut is_exist = false;
        for board_unit in board.iter_mut() {
            if board_unit[0] == price {
                board_unit[1] = size;
                is_exist = true;
                break;
            }
        }
        if is_exist == false { board.push([board_unit.price, board_unit.size]); }
    }
    // size = 0の場合には削除
    board.retain(|x| x[1] != 0.0);
    // 並び替える
    board.sort_by(|a: &[f32; 2], b| a[0].partial_cmp(&b[0]).unwrap()); // 昇順
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
    run().await;
    // test();
}