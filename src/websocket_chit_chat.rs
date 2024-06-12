use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Instant;
use tungstenite::{connect, Message};

#[derive(Serialize, Deserialize)]
struct Input {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    secret: String,
}

pub fn websocket_chit_chat(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;

    let (mut socket, response) =
        connect("wss://hackattic.com/_/ws/".to_owned() + input.token.as_str()).unwrap();

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let mut now = Instant::now();
    let mut secret = String::new();
    loop {
        let msg = socket.read().expect("Error reading message");
        match msg.to_text() {
            Ok("ping!") => {
                let elapsed = now.elapsed();

                let resp = match elapsed.as_millis() {
                    0..=1100 => String::from("700"),
                    1101..=1750 => String::from("1500"),
                    1751..=2250 => String::from("2000"),
                    2251..=2750 => String::from("2500"),
                    2751.. => String::from("3000"),
                };
                socket.send(Message::Text(resp)).unwrap();
                now = Instant::now();
            }
            Ok(s) if s.starts_with("congratulations!") => {
                let re = Regex::new("\"(.*)\"").unwrap();
                secret = String::from(re.captures(s).unwrap().get(1).unwrap().as_str());
                println!("{}", secret);
                break;
            }
            Ok(_) => (),
            _ => break,
        }
        println!("Received: {}", msg);
    }

    let output = serde_json::to_string(&Output { secret: secret })?;

    Ok(output)
}
