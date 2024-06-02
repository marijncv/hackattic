use clap::Parser;
use std::error::Error;

mod backup_restore;
mod help_me_unpack;
mod mini_miner;

const BASE_URL: &str = "https://hackattic.com/challenges/";
const ACCESS_TOKEN: &str = "?access_token=a77a711f47366f38";

#[derive(clap::ValueEnum, Clone, Copy)]
enum Challenge {
    MiniMiner,
    HelpMeUnpack,
    BackupRestore,
}

impl Challenge {
    pub fn as_str(&self) -> &str {
        match &self {
            Challenge::HelpMeUnpack => "help_me_unpack",
            Challenge::MiniMiner => "mini_miner",
            Challenge::BackupRestore => "backup_restore",
        }
    }
    pub async fn solve(&self, input: String) -> Result<String, Box<dyn Error>> {
        match &self {
            Challenge::MiniMiner => mini_miner::mini_miner(input),
            Challenge::HelpMeUnpack => help_me_unpack::help_me_unpack(input),
            Challenge::BackupRestore => backup_restore::backup_restore(input).await,
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    #[arg(short, long)]
    challenge: Challenge,

    #[arg(short, long, action)]
    playground: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let problem_url = get_problem_url(args.challenge);
    let solve_url = get_solve_url(args.challenge, args.playground);

    let input = get_input(&client, problem_url).await?;
    let output = args.challenge.solve(input).await?;

    post_output(&client, solve_url, output).await?;
    Ok(())
}

fn get_problem_url(challenge: Challenge) -> String {
    BASE_URL.to_string() + challenge.as_str() + "/problem" + ACCESS_TOKEN
}

fn get_solve_url(challenge: Challenge, playground: bool) -> String {
    let p = match playground {
        true => "&playground=1",
        false => "",
    };
    BASE_URL.to_string() + challenge.as_str() + "/solve" + ACCESS_TOKEN + p
}

async fn get_input(client: &reqwest::Client, url: String) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

async fn post_output(
    client: &reqwest::Client,
    url: String,
    body: String,
) -> Result<(), Box<dyn Error>> {
    let request = client
        .post(url)
        .body(body)
        .header("Accept", "application.json")
        .header("Content-Type", "application/json");
    let response = request.send().await?.text().await?;
    println!("{}", response);
    Ok(())
}
