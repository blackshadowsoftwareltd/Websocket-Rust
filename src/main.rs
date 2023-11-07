pub mod config;
pub mod events;
pub mod helpers;
pub mod models;

use anyhow::Result;
use config::config_ws::ws_config;
use events::handler::handle_connection;

#[tokio::main]
async fn main() {
    match ws_run().await {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {:?}", e),
    }
}

pub async fn ws_run() -> Result<()> {
    let (state, listener) = ws_config().await?;
    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }
    anyhow::Ok(())
}
