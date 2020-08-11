mod logger;

#[macro_use]
extern crate log;

#[cfg(debug_assertions)]
use dotenv::dotenv;

use std::env;

fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let amqp_host = env::var("AMQP_HOST");

    if let Ok(value) = amqp_host {
        println!("{}", value);        
    } else {
        return
    }
}
