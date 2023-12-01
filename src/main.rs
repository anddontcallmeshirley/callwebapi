use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
// use std::env;
use std::collections::HashMap;


static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

    // .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")


// to run this project, open Terminal and type:
// cargo run

// we "borrowed" code from:
// https://stackoverflow.com/questions/65757876/how-to-fix-reqwest-json-decode-errors

// this was the original API we tried:
// https://api.pro.coinbase.com/products/BTC-USD/candles

// returns data in this format:
// [[1689020640,30866.91,30880.72,30869.1,30880.72,1.67204618],[...]]
// this is a valid JSON array!

// Rust data types:
// i = integer, u = unsigned, f = float ... example: i64, f64...

// Response Items from API:
// Each bucket is an array of the following information:

// time bucket start time
// low lowest price during the bucket interval
// high highest price during the bucket interval
// open opening price (first trade) in the bucket interval
// close closing price (last trade) in the bucket interval
// volume volume of trading activity during the bucket interval

// previous struct was
// struct BitcoinPrice {
//     t: i64,
//     l: f64,
//     h: f64,
//     o: f64,
//     c: f64,
//     v: f64,
// }

#[derive(Serialize, Deserialize, Debug)]
struct BitcoinPrice {
    base: String,
    currency: String,
    amount: String
}
#[derive(Serialize, Deserialize, Debug)]
struct BitcoinPrice2 {
    base: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64
}
// amount was f64 but not valid - 30537.43500003423

// new API call is here:
// https://api.coinbase.com/v2/prices/BTC-USD/spot"
// returns data in this format:
// {"data":{"base":"BTC","currency":"USD","amount":"30512.994999960047"}}
//
// therefore we had to build 2 structs to match, the outer ApiData, and inner BitcoinPrice

#[derive(Serialize, Deserialize, Debug)]
struct ApiData {
    data: BitcoinPrice,
}
#[derive(Serialize, Deserialize, Debug)]
struct ApiData2 {
    base: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64
}

impl ApiData {
    async fn get() -> Result<Self, ExitFailure> {
        // this API call returns a more structured, named data set, a JSON object:
        let url = format!(
                    "https://api.coinbase.com/v2/prices/BTC-USD/spot"
        );
        // {
        //     "data": {
        //       "base": "BTC",
        //       "currency": "USD",
        //       "amount": "30476.89"
        //     }
        //   }

        let url = Url::parse(&*url)?;

        let res = reqwest::get(url).await?.json::<ApiData>().await?;      

        Ok(res)
    }
}

impl ApiData2 {
    async fn get() -> Result<Self, ExitFailure> {
        // this is the old API. we removed.
        type Timestamp = u64;
        let start_timestamp: Timestamp = 1682978460; //May 1, 2023 22:01:00 GMT
        let seconds_of_time: u64 = 60; //we start with 60 seconds
        let host = "api.pro.coinbase.com";
        
        let url = format!(
            "https://{}/products/BTC-USD/candles?start={}&end={}&granularity={}",
            host,
            start_timestamp.to_string(),
            start_timestamp.to_string(),
            seconds_of_time.to_string()
        );

        let url = format!(
            "https://api.coinbase.com/v2/prices/BTC-USD/spot"
);

        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await;

        println!("{:?}", response );
        
        // match response.status() {
        //     reqwest::StatusCode::OK => {
        //         println!("Success! {:?}");
        //     },
        //     reqwest::StatusCode::UNAUTHORIZED => {
        //         println!("Need to grab a new token");
        //     },
        //     _ => {
        //         panic!("Uh oh! Something unexpected happened.");
        //     },
        // };
        // https://blog.logrocket.com/making-http-requests-rust-reqwest/

        // println!("{:?}", reqwest::get(url).await? );

        let start_timestamp: Timestamp = 1682978460; //May 1, 2023 22:01:00 GMT
        let seconds_of_time: u64 = 60; //we start with 60 seconds
        let host = "api.pro.coinbase.com";
        let url = format!(
            "https://{}/products/BTC-USD/candles?start={}&end={}&granularity={}",
            host,
            start_timestamp.to_string(),
            start_timestamp.to_string(),
            seconds_of_time.to_string()
        );
        let url = Url::parse(&*url)?;
println!("A1");
//        let res = reqwest::get(url).await?.json::<ApiData2>().await?;
let client = reqwest::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()?;


    let response_body = client.get(url)
    .send()
    .await?
    .text()
    .await?;
        // let response_body = client::get(url)
        // .await
        // .unwrap()
        // .text()
        // .await;
    // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html

// println!("Response Body: {}", response_body);

// let ip = response_body
// .json::<HashMap<String, String>>()
//     .await?;

// println!("{:?}", ip);

println!("A2");
// ERROR HERE --
let res: ApiData2 = serde_json::from_str(&response_body)?;

// https://stackoverflow.com/questions/65757876/how-to-fix-reqwest-json-decode-errors
println!("A3");
        // let resA = reqwest::get(url).await?;
        // let str_body = String::from_utf8(resA)
        //            .map_err(|_| "Transformed response is not UTF-8 encoded.")?;

        // let data: Vec<Vec<f64>> = serde_json::from_str(&str_body)
        // .map_err(|_| "Failed to parse the response into the expected format.")?;

        // if let Some(record) = data.get(0) {
        //     if let Some(closing_price) = record.get(4) {
        //         println!("here's the price!");
        //         //println!(Nat::from(*closing_price as u64));
        //         println!("{}", String::from(closing_price.to_string() ) );
        //     }
        // }

        Ok(res)
    }
}

// https://stackoverflow.com/questions/65757876/how-to-fix-reqwest-json-decode-errors
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // this was old code to get stock quote using api from:
    // https://finnhub.io/api/v1/stock/profile2?symbol={}&token={}
    // let api_key = "cim68gpr01qlseds8hpgcim68gpr01qlseds8hq0".to_string();
    // let args: Vec<String> = env::args().collect();
    // let mut symbol: String = "AAPL".to_string();

    // if args.len() < 2 {
    //     println!("Since you didn't specify a company symbol, it has defaulted to AAPL.");
    // } else {
    //     symbol = args[1].clone();
    // }

    // let res = CompanyQuote::get(&symbol, &api_key).await?;
    // println!("{}'s current price is: {}", symbol, res.c);
    // println!("the whole object is: {:?}", res);

    // call the get function of the 
    let res = ApiData::get().await?;
    println!("res is: {:?}", res);
    // result so far is:
    // ApiData { data: BitcoinPrice { base: "BTC", currency: "USD", amount: "30554.615" } }
    println!("res.data.amount is {:?}", res.data.amount );


    let res2 = ApiData2::get().await?;
    println!("res2 is {:?}", res2 );

    // notes for future reference,
    // we also tried this, and it returns results,
    // let res = client.get("https://api.pro.coinbase.com/products/BTC-USD/candles?start=1682978460&end=1682978480&granularity=60")
    // .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
    // .header(CONTENT_TYPE, "application/json")
    // .send()
    // .await
    // .expect("failed to get response")
    // .text()
    // .await
    // .expect("failed to get payload");

    // let res: Result<reqwest::Response, reqwest::Error> = client.get(url).send().await.expect("failed to get response").text().await.expect("failed to get payload");
    // https://stackoverflow.com/questions/70468338/how-to-get-body-of-response-with-reqwest

    // println!("{}", res);

    // returns this
    // [[1682978460,27976.02,28000.25,27999.15,27985.63,6.16489315]]

    // Have to save this into the struct, then can parse, but how?

    // https://stackoverflow.com/questions/67846025/rust-json-how-to-convert-str-to-json-response-in-rust
    // let input = r#"{"index":0,"name":"AB/CDE/FG/402/test_int4","sts":"on","time":"2021-06-05 03:28:24.044284300 UTC","value":8}"#;
    
    // let mut object: Data = serde_json::from_str(input).unwrap();
    
    // see code at https://gist.github.com/chris-gong/b24130f5ea0c6c93e3c24bfb4aca27fd

    // difference between square brackets and curly brackets?
    // The [] square brackets produce a list/array.
    // The {} curly brackets produce an object with key/value pairs.
    // The list can then be a value of a key/value pair.

    Ok(())
}