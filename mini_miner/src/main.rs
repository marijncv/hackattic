use std::error::Error;
use serde::{Deserialize, Serialize};
use sha2::{Digest};

use rand::{distributions::Alphanumeric, Rng};

const BASE_URL: &str = "https://hackattic.com/challenges/mini_miner/";
const ACCESS_TOKEN: &str = "access_token=a77a711f47366f38";

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Data{
    Int(i64),
    Str(String)
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    data: Vec<Vec<Data>>,
    nonce: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    difficulty: u32,
    block: Block,
}

#[derive(Serialize, Deserialize)]
struct Output {
    nonce: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let input = get_input(&client).await?;
    let mut block = input.block;

    let mut found = false;

    while !found {
        let nonce: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        block.nonce = Some(nonce);
        
        if get_leading_zeroes(&block).unwrap() == input.difficulty {
            found = true
        }
    }
    let output = Output{nonce: block.nonce.unwrap()};
    
    send_result(&client, output).await?;

    Ok(())
}

async fn get_input(client: &reqwest::Client) -> Result<Input, Box<dyn Error>>{
    let url = BASE_URL.to_string() + "problem?" + ACCESS_TOKEN;
    let response = &client.get(url).send().await?.text().await?;
    let input = serde_json::from_str::<Input>(&response)?;
    Ok(input)
}

fn get_leading_zeroes(block: &Block) -> Result<u32, Box<dyn Error>>{
    let hash: [u8;32] = sha2::Sha256::digest(serde_json::to_string(&block)?).into();
    let leading_zeroes = hash.iter().try_fold(0, |acc, n| {
        if *n == u8::from(0) {
            Ok(acc + 8)
        } else {
            Err(acc + n.leading_zeros())
        }
    }).unwrap_or_else(|e| e);
    Ok(leading_zeroes)
}

async fn send_result(client: &reqwest::Client, output: Output)  -> Result<(), Box<dyn Error>>{
    let url = BASE_URL.to_string() + "solve?" + ACCESS_TOKEN;
    let body = serde_json::to_string(&output)?;
    let request = client.post(url).body(body).header("Accept", "application.json").header("Content-Type", "application/json");
    let response = request.send().await?.text().await?;
    println!("{}", response);
    Ok(())
}