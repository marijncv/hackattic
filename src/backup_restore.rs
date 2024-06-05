use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::process::Command;
use std::{thread, time};
use tokio_postgres::{Client, NoTls};

const SQL_DUMP_FILE: &str = "dump.sql";

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

    write_sql_dump_to_file(input)?;

    let client = setup_postgres().await?;

    let query_result = client
        .query(
            "SELECT ssn FROM public.criminal_records WHERE status = 'alive'",
            &[],
        )
        .await?;

    let output = Output {
        alive_ssns: query_result.iter().map(|r| r.get(0)).collect(),
    };

    cleanup_side_effects()?;

    Ok(serde_json::to_string(&output)?)
}

fn write_sql_dump_to_file(input: Input) -> Result<(), Box<dyn Error>> {
    let b64_decoded = BASE64_STANDARD.decode(input.dump)?;

    let mut decoder = GzDecoder::new(b64_decoded.as_slice());
    let mut dump = String::new();
    decoder.read_to_string(&mut dump).unwrap();

    fs::write(SQL_DUMP_FILE, dump)?;
    Ok(())
}

fn cleanup_side_effects() -> Result<(), Box<dyn Error>> {
    Command::new("sh")
        .arg("-c")
        .arg("docker rm -f postgresql")
        .status()?;

    fs::remove_file(SQL_DUMP_FILE)?;
    Ok(())
}

async fn setup_postgres() -> Result<Client, Box<dyn Error>> {
    Command::new("sh")
        .arg("-c")
        .arg("docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_HOST_AUTH_METHOD=trust -p 5432:5432 --name postgresql postgres:12-alpine")
        .status()?;

    thread::sleep(time::Duration::from_secs(1));

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "psql -h localhost -U postgres postgres -f {}",
            SQL_DUMP_FILE
        ))
        .status()?;

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
