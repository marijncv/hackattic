use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

#[derive(Serialize, Deserialize)]
struct RequiredData {
    domain: String,
    serial_number: String,
    country: String,
}

#[derive(Serialize, Deserialize)]
struct Input {
    private_key: String,
    required_data: RequiredData,
}

#[derive(Serialize, Deserialize)]
struct Output {
    certificate: String,
}

pub async fn tales_of_ssl(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;

    let private_key = "-----BEGIN RSA PRIVATE KEY-----\n".to_string()
        + input.private_key.as_str()
        + "\n-----END RSA PRIVATE KEY-----";
    fs::write("cert.key", private_key)?;

    let mut cmd = Command::new("openssl")
        .args(["req", "-new", "-key", "cert.key", "-out", "cert.csr"])
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = cmd.stdin.take().unwrap();
    stdin
        .write_all(
            format!(
                "\n{}\n\n\n\n{}\n\n\n\n",
                input.required_data.country, input.required_data.domain
            )
            .as_bytes(),
        )
        .unwrap();

    Command::new("openssl")
        .args([
            "x509",
            "-req",
            "-in",
            "cert.csr",
            "-signkey",
            "cert.key",
            "-out",
            "cert.crt",
            "-outform",
            "DER",
            "-set_serial",
            input.required_data.serial_number.as_str(),
        ])
        .status()?;

    let cert = BASE64_STANDARD.encode(fs::read("cert.crt").unwrap());

    let output = serde_json::to_string(&Output { certificate: cert })?;

    Ok(output)
}
