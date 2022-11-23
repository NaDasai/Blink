
// let df_columns = ['open_time', 'close_time', 'open', 'high', 'low', 'close', 
//               'volume', 'quote_asset_volume', 'num_trades', 'taker_buy_base_asset_volume', 
//               'taker_buy_quote_asset_volume', 'ignore', 'open_timestamp', 'close_timestamp'];
// tokio::time::sleep_until (time_to_hackathon_date).await;


use error_chain::error_chain;
use chrono::{DateTime, Utc};
use std::io::{Write};
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[derive(Serialize, Deserialize)]
struct Binance {
    symbol: String,
    price : String,
}
#[derive(Serialize, Deserialize)]
struct Coingecko {
bitcoin: HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<()> {

    // Get prices
    
    let binance_res = reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT").await?;
    // https://api.binance.com/api/v3/exchangeInfo?symbol=BNBBTC
    //let res = reqwest::get("https://api.binance.com/api/v3/exchangeInfo?symbol=BTCUSDT").await?;
    // ?symbols=["BNBBTC","ETHUSDT"]
    let coingecko_res = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").await?;
    
    let binance_body = binance_res.text().await?;
    let coingecko_body = coingecko_res.text().await?;   
    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let binance_json: Binance = serde_json::from_str(&binance_body).expect("Nothing!");
    let coingecko_json: Coingecko = serde_json::from_str(&coingecko_body).expect("Nothing!");

    // Get current time

    let now: DateTime<Utc> = Utc::now();

    // Compare values

    println!("{}\nBinance:\n{}\nCoingecko:\n{}", now.format("%a %b %e %T %Y"), binance_json.price, coingecko_json.bitcoin.get("usd").unwrap());
    
    // We are saving results to file (create if doesn't exist)
    
    let file_name = "blinks.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;
    //Wed Nov 16 18:37:54 2022
    writeln!(file, "{}\nBinance: {}\nCoingecko: {}", now.format("%a %b %e %T %Y"), binance_json.price, coingecko_json.bitcoin.get("usd").unwrap())?; // writing using the macro 'writeln!'


    // We are using QuickNode API on Matic main network to get a faster result

    let transport = web3::transports::Http::new("https://skilled-young-choice.matic.discover.quiknode.pro/62ff487298631c3d207ced949a588db4acc9aa9f/").expect("No transport!");
    let web3 = web3::Web3::new(transport);
    let current_block = web3.eth().block_number().await.unwrap();
    println!("The current block is : {:#?}", current_block);
    let recommended_gas_price = web3.eth().gas_price().await.unwrap();
    println!("Recommended gas price is : {:#?}", recommended_gas_price);

    //my_call_request = web3::types::CallRequest::new();
    //let estimate_gas = web3.eth().estimate_gas(my_call_request, current_block).await;
  
    Ok(())

}