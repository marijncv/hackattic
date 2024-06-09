mod backup_restore;
mod brute_force_zip;
mod challenges;
mod collision_course;
mod help_me_unpack;
mod jotting_jwts;
mod mini_miner;
mod password_hashing;
mod servers;
mod util;

use challenges::Challenge;
use clap::Parser;
use servers::Server;
use std::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    #[arg(short, long)]
    challenge: Option<Challenge>,

    #[clap(value_enum)]
    #[arg(short, long)]
    server: Option<Server>,

    #[arg(short, long, action)]
    playground: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let challenge = args.challenge;
    let server = args.server;
    match challenge {
        Some(c) => {
            let problem_url = util::get_problem_url(c);
            let solve_url = util::get_solve_url(c, args.playground);

            let input = util::get_input(&client, problem_url).await?;
            let output = c.solve(input).await?;
            util::post_output(&client, solve_url, output).await?;
        }
        _ => (),
    }

    match server {
        Some(s) => {
            s.start()?;
        }
        _ => (),
    }

    Ok(())
}
