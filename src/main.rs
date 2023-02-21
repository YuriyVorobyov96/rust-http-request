use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();

    let resp200 = client.get("https://httpbin.org/get")
    .header(CONTENT_TYPE, "application/json")
    .send()
    .await?
    .json::<GETAPIResponse>()
    .await?;
    
    println!("{:#?}", resp200);

    let mut map = HashMap::new();
    map.insert("key", "value");

    let resp_json = client.post("https://httpbin.org/post")
    .json(&map)
    .send()
    .await?
    .json::<JSONResponse>()
    .await?;

    println!("{:#?}", resp_json);

    Ok(())
}
