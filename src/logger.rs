use log::LevelFilter;

use chrono::Local;

use std::thread::current;

use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;

use std::str::FromStr;

use crate::CONFIG;

pub fn init() {
    let level = LevelFilter::from_str(&CONFIG.log_level)
                            .unwrap_or(LevelFilter::Info);

    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack)
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    Dispatch::new()
        .format(move |out, message, record| {
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
        .apply()
        .unwrap_or_else(|err| {
            error!(
                "Could not register Fern as logger, falling back to default implementation, {:?}",
                err
            )
        });
}
