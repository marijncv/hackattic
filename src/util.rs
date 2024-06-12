use crate::challenges::Challenge;
use std::error::Error;

const BASE_URL: &str = "https://hackattic.com/challenges/";
const ACCESS_TOKEN: &str = "?access_token=a77a711f47366f38";

pub fn get_problem_url(challenge: Challenge) -> String {
    BASE_URL.to_string() + challenge.as_str() + "/problem" + ACCESS_TOKEN
}

pub fn get_solve_url(challenge: Challenge, playground: bool) -> String {
    let p = match playground {
        true => "&playground=1",
        false => "",
    };
    BASE_URL.to_string() + challenge.as_str() + "/solve" + ACCESS_TOKEN + p
}

pub async fn get_input(url: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

pub async fn post_output(url: String, body: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request = client
        .post(url)
        .body(body)
        .header("Accept", "application.json")
        .header("Content-Type", "application/json");
    let response = request.send().await?.text().await?;
    println!("{}", response);
    Ok(())
}
