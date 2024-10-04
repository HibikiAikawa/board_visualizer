use bybit::client::fetch;

mod coincheck;
mod hyperliquid;
mod bybit;
mod structs;

const MAX_BOARD_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    // coincheck::client::run(MAX_BOARD_SIZE).await;
    // hyperliquid::client::run(MAX_BOARD_SIZE).await;
    // bybit::client::run(MAX_BOARD_SIZE).await;
    let max_board_size = 10;
    let save_time_min = 1;
    let symbol = "BTCUSDT".to_string();
    let instrument = "perp".to_string();
    let dir_path = ".".to_string();
    bybit::client::fetch(max_board_size, save_time_min, symbol, instrument, dir_path).await;
}
    