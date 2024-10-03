mod coincheck;
mod hyperliquid;
mod structs;

const MAX_BOARD_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    // coincheck::client::run(MAX_BOARD_SIZE).await;
    hyperliquid::client::run(MAX_BOARD_SIZE).await;
}
    