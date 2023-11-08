pub mod config;
pub mod events;
pub mod helpers;
pub mod models;

use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use config::config_ws::ws_config;
use events::handler::handle_connection;
use helpers::{
    function::user::{init_current_user, init_users},
    types::PeerMap,
};

#[tokio::main]
async fn main() {
    init_current_user();
    init_users();
    match ws_run().await {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {:?}", e),
    }
}

pub async fn ws_run() -> Result<()> {
    let listener = ws_config().await?;
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }
    anyhow::Ok(())
}
