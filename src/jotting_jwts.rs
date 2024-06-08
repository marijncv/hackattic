use base64::Engine;
use base64::{alphabet, engine};
use hmac_sha256::HMAC;
use rouille::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::Read;
use std::sync::Mutex;
use std::sync::Arc;

use std::time::{SystemTime, UNIX_EPOCH};

const B64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::NO_PAD);

#[derive(Serialize, Deserialize)]
struct Input {
    jwt_secret: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    app_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeResponse {
    solution: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    append: Option<String>,
    iss: Option<String>,
    aud: Option<String>,
    exp: Option<u64>,
    nbf: Option<u64>,
}

#[derive(Debug)]
struct Jwt {
    header: String,
    payload: String,
    signature: String,
}

pub async fn jotting_jwts(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;
    let output = serde_json::to_string(&Output {
        app_url: "http://test.marijncv.com/test".to_string(),
    })?;

    fs::write("jwt-secret", input.jwt_secret)?;

    Ok(output)
}

fn sign_jwt(header: &String, payload: &String, jwt_secret: &String) -> String {
    B64_ENGINE.encode(HMAC::mac(header.to_owned() + "." + &payload, jwt_secret))
}

fn verify_payload(payload: &Payload) -> bool {
    let start = SystemTime::now();
    let now = start
        .duration_since(UNIX_EPOCH).unwrap().as_secs();

    println!("time: {:?}", now);

    match payload.exp {
        Some(exp) => {
            if now > exp {
                return false
            }
        },
        None => ()
    }

    match payload.nbf {
        Some(nbf) => {
            if now < nbf {
                return false
            }
        },
        None => ()
    }

    true
}

pub fn start_server() {
    let result_vec: Vec<String> = Vec::new();
    let result = Arc::new(Mutex::new(result_vec));


    let server = rouille::Server::new("0.0.0.0:8001", move |request| {
        let result = Arc::clone(&result);

        let jwt_secret = fs::read_to_string("jwt-secret").unwrap_or("".to_string());

        let jwt = match request.data() {
            Some(mut b) => {
                let mut body = String::new();
                b.read_to_string(&mut body).unwrap();
                let mut token = body.splitn(3, ".");
                Some(Jwt {
                    header: token.next().unwrap().to_string(),
                    payload: token.next().unwrap().to_string(),
                    signature: token.next().unwrap().to_string(),
                })
            }
            _ => None,
        }
        .unwrap();

        let my_signed_jwt = sign_jwt(&jwt.header, &jwt.payload, &jwt_secret);
        let payload: Payload = serde_json::from_str::<Payload>(
            &String::from_utf8(B64_ENGINE.decode(jwt.payload.as_str()).unwrap()).unwrap(),
        )
        .unwrap();
        println!("{:?}", payload);

        let response = match verify_payload(&payload) && my_signed_jwt == jwt.signature {
            true => {
                match payload.append {
                    Some(a) => {
                        let mut r = result.lock().unwrap();
                        r.push(a.to_owned());
                        println!("{:?}",r);
                        Response::text("")
                    }
                    None => {
                        let r = result.lock().unwrap().concat();
    
                        let challenge_response = ChallengeResponse { solution: r };
                        println!("{:?}", challenge_response);
                        Response::json(&challenge_response)
                    }
                }
            },
            false => Response::text("")
        };
        response
    })
    .unwrap();
    server.run()
}
