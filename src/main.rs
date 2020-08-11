mod logger;

#[macro_use]
extern crate log;

#[cfg(debug_assertions)]
use dotenv::dotenv;

use std::env;

use lapin::{
    Connection, ConnectionProperties, Result
};

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    logger::start_logger();

    let amqp_host = env::var("AMQP_HOST");

    if amqp_host.is_err() {
        warn!("Could not find AMQP_HOST variable, falling back to local connection")
    }

    let amqp_host = amqp_host.unwrap_or("amqp://127.0.0.1:5672/".into());

    let connection = Connection::connect(
        &amqp_host, 
        ConnectionProperties::default()
    ).await?;

    Ok(())
}