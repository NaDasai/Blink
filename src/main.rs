
// let df_columns = ['open_time', 'close_time', 'open', 'high', 'low', 'close', 
//               'volume', 'quote_asset_volume', 'num_trades', 'taker_buy_base_asset_volume', 
//               'taker_buy_quote_asset_volume', 'ignore', 'open_timestamp', 'close_timestamp'];


use error_chain::error_chain;
use chrono::{DateTime, Utc};
use std::io::{Write};
use std::fs::OpenOptions;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let res = reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT").await?;
    // https://api.binance.com/api/v3/exchangeInfo?symbol=BNBBTC
    //let res = reqwest::get("https://api.binance.com/api/v3/exchangeInfo?symbol=BTCUSDT").await?;
    // ?symbols=["BNBBTC","ETHUSDT"]
    
    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let now: DateTime<Utc> = Utc::now();
    println!("UTC now is: {}", now);

    let body = res.text().await?;
    println!("Body:\n{}", body);

    let file_name = "blinks.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;
    //Wed Nov 16 18:37:54 2022
    writeln!(file, "{}\n{}", now.format("%a %b %e %T %Y"), body)?; // writing using the macro 'writeln!'

    
    Ok(())

}