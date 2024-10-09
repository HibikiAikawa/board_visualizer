mod coincheck;
mod structs;

const MAX_BOARD_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    let mut coincheck_rx = coincheck::client::run(MAX_BOARD_SIZE);

    loop {
        tokio::select! {
            Some(board) = coincheck_rx.recv() => {
                println!("{:?}", board);
            }
        }
    }
}
    