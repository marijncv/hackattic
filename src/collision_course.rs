use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Input {
    include: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    files: Vec<String>,
}

pub async fn collision_course(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;

    fs::create_dir_all("collision-course")?;

    fs::write("collision-course/prefix.txt", input.include)?;

    Command::new("sh")
        .current_dir("collision-course")
        .arg("-c")
        .arg("sudo docker run --rm -it -v $PWD:/collision-course -w /collision-course brimstone/fastcoll --prefixfile prefix.txt -o col1.bin col2.bin")
        .status()?;

    let output = serde_json::to_string(&Output {
        files: vec![
            BASE64_STANDARD.encode(fs::read("./collision-course/col1.bin")?),
            BASE64_STANDARD.encode(fs::read("./collision-course/col2.bin")?),
        ],
    })?;

    Ok(output)
}
