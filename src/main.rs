mod logger;

mod queue_processor;
mod mailer;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use std::env::var;

use tokio_amqp::LapinTokioExt;
use lapin::{
    Connection, ConnectionProperties, Result
};

use queue_processor::{setup_qos, declare_queue, create_consumer, setup_listener};

lazy_static! {
    
    static ref ID: String = {
        var("ID").unwrap_or_else(|_| {
            warn!("Could not find ID variable, falling back to development id");
            "dev-0".into()
        })
    };

    static ref AMQP_HOST: String = {
        var("AMQP_HOST").unwrap_or_else(|_| {
            warn!("Could not find AMQP_HOST variable, falling back to local connection");
            "amqp://127.0.0.1:5672/%2f".into()
        })
    };

    static ref QUEUE: String = {
        var("QUEUE").unwrap_or_else(|_| {
            warn!("Could not find QUEUE variable, falling back to default queue");
            "emails".into()
        })
    };

}

#[tokio::main]
async fn main() -> Result<()> {
    // .env support for debug environments
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    logger::init();

    let connection = Connection::connect(
        &AMQP_HOST, 
        ConnectionProperties::default().with_tokio()
    ).await?;

    let channel = connection.create_channel().await?;

    setup_qos(&channel).await?;
    declare_queue(&QUEUE, &channel).await?;

    setup_listener(
        create_consumer(
            &QUEUE, 
            &ID,
            &channel
        ).await?
    ).await;

    connection.close(320, "Shutdown").await?;

    Ok(())
}