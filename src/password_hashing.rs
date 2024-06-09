use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use hmac_sha256::HMAC;
use pbkdf2::hmac::Hmac;
use pbkdf2::pbkdf2;
use scrypt::Params;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;
use std::error::Error;

use scrypt::scrypt;

#[derive(Serialize, Deserialize)]
struct Pbkdf2 {
    hash: String,
    rounds: u32,
}

#[derive(Serialize, Deserialize)]
struct ScryptInput {
    N: u32,
    p: u32,
    r: u32,
    buflen: usize,
    _control: String,
}

#[derive(Serialize, Deserialize)]
struct Input {
    password: String,
    salt: String,
    pbkdf2: Pbkdf2,
    scrypt: ScryptInput,
}

#[derive(Serialize, Deserialize)]
struct Output {
    sha256: String,
    hmac: String,
    pbkdf2: String,
    scrypt: String,
}

pub fn password_hashing(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;

    let password = input.password.as_bytes();
    let salt = BASE64_STANDARD.decode(&input.salt)?;

    let sha = Sha256::digest(password);
    let hmac = HMAC::mac(password, salt.clone());

    let mut pbkdf2_output = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(
        password,
        salt.as_slice(),
        input.pbkdf2.rounds,
        &mut pbkdf2_output,
    )?;

    let scrypt_params = Params::new(
        input.scrypt.N.checked_ilog2().unwrap().try_into()?,
        input.scrypt.r,
        input.scrypt.p,
        input.scrypt.buflen,
    )?;
    let mut scrypt_output = [0u8; 32];
    scrypt(
        password,
        &salt.as_slice(),
        &scrypt_params,
        &mut scrypt_output,
    )?;

    let output = Output {
        sha256: hex::encode(sha),
        hmac: hex::encode(hmac),
        pbkdf2: hex::encode(pbkdf2_output),
        scrypt: hex::encode(scrypt_output),
    };

    Ok(serde_json::to_string(&output)?)
}
