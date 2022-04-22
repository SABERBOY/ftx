use dotenv::dotenv;
use ftx::options::Options;
use ftx::ws::Result;
use ftx::ws::{Channel, Data, Orderbook, Ws};
use futures::stream::StreamExt;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let env=Options::from_env_us();
    let mut websocket = Ws::connect(env).await?;

    let market = String::from("BTC-PERP");
    let mut orderbook = Orderbook::new(market.to_owned());

    websocket
        .subscribe(vec![
            Channel::Trades(market.to_owned()),
            Channel::Orderbook(market.to_owned()),
        ])
        .await?;

    loop {
        let data = websocket.next().await.expect("No data received")?;

        match data {
            (_, Data::Trade(trade)) => {
                println!(
                    "\n{:?} {} {} at {} - liquidation = {}",
                    trade.side, trade.size, market, trade.price, trade.liquidation
                );
            }
            (_, Data::OrderbookData(orderbook_data)) => {
                orderbook.update(&orderbook_data);
                print!("."); // To signify orderbook update
                io::stdout().flush().unwrap(); // Emits the output immediately
            }
            _ => panic!("Unexpected data type"),
        }
    }
}
