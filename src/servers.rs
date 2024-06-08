use crate::jotting_jwts;
use std::error::Error;

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Server {
    JottingJwts,
}

impl Server {
    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        match &self {
            Server::JottingJwts => Ok(jotting_jwts::start_server()),
        }
    }
}
