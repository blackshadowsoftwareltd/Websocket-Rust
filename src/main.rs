pub mod connect;
pub mod events;
pub mod helpers;
pub mod models;
pub mod run;

use crate::run::run_ws;

#[tokio::main]
async fn main() {
    run_ws().await;
}
