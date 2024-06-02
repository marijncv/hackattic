use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub fn help_me_unpack(input: String) -> Result<String, Box<dyn Error>> {
    let bytes = get_bytes(input)?;

    let unpack = Unpack {
        int: i32::from_le_bytes(bytes[0..4].try_into()?),
        uint: u32::from_le_bytes(bytes[4..8].try_into()?),
        short: i16::from_le_bytes(bytes[8..10].try_into()?),
        float: f32::from_le_bytes(bytes[12..16].try_into()?),
        double: f64::from_le_bytes(bytes[16..24].try_into()?),
        big_endian_double: f64::from_be_bytes(bytes[24..32].try_into()?),
    };

    let output = serde_json::to_string(&unpack)?;

    Ok(output)
}

fn get_bytes(input: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let pack = serde_json::from_str::<Pack>(&input);
    Ok(BASE64_STANDARD.decode(pack?.bytes)?)
}
