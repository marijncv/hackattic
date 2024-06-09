use crate::backup_restore;
use crate::brute_force_zip;
use crate::help_me_unpack;
use crate::jotting_jwts;
use crate::mini_miner;
use crate::password_hashing;
use std::error::Error;

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Challenge {
    MiniMiner,
    HelpMeUnpack,
    BackupRestore,
    BruteForceZip,
    PasswordHashing,
    JottingJwts,
}

impl Challenge {
    pub fn as_str(&self) -> &str {
        match &self {
            Challenge::HelpMeUnpack => "help_me_unpack",
            Challenge::MiniMiner => "mini_miner",
            Challenge::BackupRestore => "backup_restore",
            Challenge::BruteForceZip => "brute_force_zip",
            Challenge::PasswordHashing => "password_hashing",
            Challenge::JottingJwts => "jotting_jwts",
        }
    }
    pub async fn solve(&self, input: String) -> Result<String, Box<dyn Error>> {
        match &self {
            Challenge::MiniMiner => mini_miner::mini_miner(input),
            Challenge::HelpMeUnpack => help_me_unpack::help_me_unpack(input),
            Challenge::BackupRestore => backup_restore::backup_restore(input).await,
            Challenge::BruteForceZip => brute_force_zip::brute_force_zip(input).await,
            Challenge::PasswordHashing => password_hashing::password_hashing(input),
            Challenge::JottingJwts => jotting_jwts::jotting_jwts(input).await,
        }
    }
}