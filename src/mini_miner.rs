use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::error::Error;

use rand::{distributions::Alphanumeric, Rng};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Data {
    Int(i64),
    Str(String),
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

pub fn mini_miner(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;
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
    let output = Output {
        nonce: block.nonce.unwrap(),
    };

    Ok(serde_json::to_string(&output)?)
}

fn get_leading_zeroes(block: &Block) -> Result<u32, Box<dyn Error>> {
    let hash: [u8; 32] = sha2::Sha256::digest(serde_json::to_string(&block)?).into();
    let leading_zeroes = hash
        .iter()
        .try_fold(0, |acc, n| {
            if *n == u8::from(0) {
                Ok(acc + 8)
            } else {
                Err(acc + n.leading_zeros())
            }
        })
        .unwrap_or_else(|e| e);
    Ok(leading_zeroes)
}
