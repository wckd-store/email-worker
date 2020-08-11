use log::LevelFilter;

use chrono::Local;

use std::thread::current;

use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;

use std::env::var;
use std::str::FromStr;

const DEFAULT_LEVEL: LevelFilter = LevelFilter::Info;

pub fn start_logger() {
    let raw_level = var("LOG_LEVEL").unwrap_or(DEFAULT_LEVEL.to_string());
    let level = LevelFilter::from_str(raw_level.as_str()).unwrap_or(DEFAULT_LEVEL);

    let colors = ColoredLevelConfig::new()
                .trace(Color::BrightBlack)
                .debug(Color::Cyan)
                .info(Color::Green)
                .warn(Color::Yellow)
                .error(Color::Red);

    let logger_result = Dispatch::new()
        .format(move | out, message, record | {
            out.finish(format_args!(
                "[{}] {:<5} from \x1B[{}m{}, {}\x1B[0m: {}",
                
                Local::now().format("%H:%M:%S.%3f"), 
                colors.color(record.level()),

                Color::BrightBlack.to_fg_str(),
                current().name().unwrap_or("unknown"),
                record.target(),

                message
            ))
        })

        .level(level)

        .chain(std::io::stdout())

        .apply();

    if let Err(err) = logger_result {
        error!("Could not register Fern as logger, falling back to default implementation, {:?}", err)
    }
}