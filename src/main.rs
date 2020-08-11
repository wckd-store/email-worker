mod logger;

#[macro_use]
extern crate log;

#[cfg(debug_assertions)]
use dotenv::dotenv;

use std::env;

fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    if let Err(err) = logger::start_logger(log::LevelFilter::Debug) {
        println!("Could not start logger, {:?}", err);
    }

    let amqp_host = env::var("AMQP_HOST");

    if let Ok(value) = amqp_host {
        info!("AMQP Host: {}", value);        
    } else {
        return
    }
}
