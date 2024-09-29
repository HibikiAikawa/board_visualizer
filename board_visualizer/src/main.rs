mod coincheck;
mod structs;

#[tokio::main]
async fn main() {
    // coincheck::client::run().await;
    structs::BoardUnit{price: 0.0, size: 0.0};
}