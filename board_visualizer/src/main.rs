mod coincheck;
mod structs;

#[tokio::main]
async fn main() {
    coincheck::client::run().await;
}