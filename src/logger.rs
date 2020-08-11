pub use log::LevelFilter;

use chrono::Local;

use log::SetLoggerError;
use std::thread::current;

use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;

pub fn start_logger(level: LevelFilter) -> Result<(), SetLoggerError> {
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
        .level_for("mio", log::LevelFilter::Warn)

        .chain(std::io::stdout())

        .apply()?;

    Ok(())
}