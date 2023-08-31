use reqwest;
use serde::Deserialize;
// use std::{fs::File, io::{BufWriter, Write}};

#[derive(Deserialize, Debug)]
struct Order {
    platinum: i16,
    quantity: i16,
    order_type: String,
    visible: bool,
    mod_rank: i8,
    region: String
}

#[derive(Deserialize, Debug)]
struct Payload {
    orders: Vec<Order>
}

#[derive(Deserialize, Debug)]
struct Obj {
    payload: Payload
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let request_url = format!("https://api.warframe.market/v1/items/{item}/orders", item = "transient_fortitude");
    let client: reqwest::Client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header("ACCEPT", "application/json")
        .header("LANGUAGE", "en")
        .send()
        .await?;
    
    // let file = File::create("response.json")?;
    // let mut writer = BufWriter::new(file);
    let orderlist = &response.json::<Obj>().await?;

    for order in &orderlist.payload.orders {
        println!("{:#?}", order);
    }

    // println!("{:#?}", orderlist.payload.orders);
    Ok(())
}
