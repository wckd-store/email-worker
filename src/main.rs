mod logger;

mod queue_processor;
mod mailer;

mod config;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

use tokio_amqp::LapinTokioExt;
use lapin::{
    Connection, ConnectionProperties, Result as LapinResult
};

use queue_processor::{setup_qos, declare_queue, create_consumer, setup_listener};

lazy_static! {
    
    pub static ref CONFIG: config::Config = {
        #[cfg(debug_assertions)] // .env support for debug environments
        dotenv::dotenv().ok();

        envy::from_env::<config::Config>().expect("Could not load environment variables")
    };

}

#[tokio::main]
async fn main() -> LapinResult<()> {
    logger::init();

    mailer::list_templates();

    let connection = Connection::connect(
        &CONFIG.amqp_host, 
        ConnectionProperties::default().with_tokio()
    ).await?;

    let channel = connection.create_channel().await?;

    setup_qos(&channel).await?;
    declare_queue(&CONFIG.queue, &channel).await?;

    setup_listener(create_consumer(
        &CONFIG.queue, 
        &CONFIG.id,
        &channel
    ).await?).await;

    connection.close(320, "Shutdown").await?;

    Ok(())
}