mod logger;
mod mailer;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use std::env;

use tokio_amqp::LapinTokioExt;
use lapin::{
    Connection, ConnectionProperties, Result
};

#[tokio::main]
async fn main() -> Result<()> {
    // .env support for debug environments
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    logger::init();

    let amqp_host = env::var("AMQP_HOST").unwrap_or_else(|_| {
        warn!("Could not find AMQP_HOST variable, falling back to local connection");
        "amqp://127.0.0.1:5672/%2f".into()
    });

    let connection = Connection::connect(
        &amqp_host, 
        ConnectionProperties::default().with_tokio()
    ).await?;

    info!("Connected? {:?}", connection.status().connected());

    mailer::send_mail();

    Ok(())
}