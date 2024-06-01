use std::error::Error;
use serde::{Deserialize, Serialize};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

const BASE_URL: &str = "https://hackattic.com/challenges/help_me_unpack/";
const ACCESS_TOKEN: &str = "access_token=a77a711f47366f38";

#[derive(Serialize, Deserialize)]
struct Pack {
    bytes: String,
}

#[derive(Serialize, Deserialize)]
struct Unpack {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let bytes = get_bytes(&client).await?;

    let unpack = Unpack{
        int: i32::from_le_bytes(bytes[0..4].try_into()?),
        uint: u32::from_le_bytes(bytes[4..8].try_into()?),
        short: i16::from_le_bytes(bytes[8..10].try_into()?),
        float: f32::from_le_bytes(bytes[12..16].try_into()?),
        double: f64::from_le_bytes(bytes[16..24].try_into()?),
        big_endian_double: f64::from_be_bytes(bytes[24..32].try_into()?),
    };

    send_result(&client, unpack).await?;

    Ok(())
}

async fn get_bytes(client: &reqwest::Client) -> Result<Vec<u8>, Box<dyn Error>>{
    let url = BASE_URL.to_string() + "problem?" + ACCESS_TOKEN; 
    let pack = serde_json::from_str::<Pack>(&client.get(url).send().await?.text().await?);
    Ok(BASE64_STANDARD.decode(pack?.bytes)?)
}

async fn send_result(client: &reqwest::Client, unpack: Unpack)  -> Result<(), Box<dyn Error>>{
    let url = BASE_URL.to_string() + "solve?" + ACCESS_TOKEN;
    let body = serde_json::to_string(&unpack)?;
    let request = client.post(url).body(body).header("Accept", "application.json").header("Content-Type", "application/json");
    let response = request.send().await?.text().await?;
    println!("{}", response);
    Ok(())
}