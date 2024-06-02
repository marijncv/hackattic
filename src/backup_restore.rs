use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use flate2::read::GzDecoder;
use tokio_postgres::NoTls;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;
use std::process::Command;
use std::{thread, time};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    dump: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    alive_ssns: Vec<String>,
}

pub async fn backup_restore(input: String) -> Result<String, Box<dyn Error>> {
    let input: Input = serde_json::from_str::<Input>(&input)?;

    let b64_decoded = BASE64_STANDARD.decode(input.dump)?;

    let mut decoder = GzDecoder::new(b64_decoded.as_slice());
    let mut dump = String::new();
    decoder.read_to_string(&mut dump).unwrap();

    fs::write("dump.sql", dump)?;

    Command::new("sh")
        .arg("-c")
        .arg("docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_HOST_AUTH_METHOD=trust -p 5432:5432 --name postgresql postgres:12-alpine")
        .status()?;

    thread::sleep(time::Duration::from_secs(1));

    Command::new("sh")
        .arg("-c")
        .arg("psql -h localhost -U postgres postgres -f dump.sql")
        .status()?;
    
    let (client, connection) = 
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query_result = client.query(
        "SELECT ssn FROM public.criminal_records WHERE status = 'alive'",
        &[],
    ).await?;

    let alive_ssns: Vec<String> = query_result.iter().map(|r| r.get(0)).collect();

    Command::new("sh")
        .arg("-c")
        .arg("docker stop postgresql")
        .status()?;

    Command::new("sh")
        .arg("-c")
        .arg("docker rm postgresql")
        .status()?;

    fs::remove_file("dump.sql")?;

    let output = Output {
        alive_ssns: alive_ssns,
    };

    Ok(serde_json::to_string(&output)?)
}


