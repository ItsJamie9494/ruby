use std::io::stderr;

use ::log::LevelFilter;
use chrono::Utc;
use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch, InitError,
};

#[derive(Default)]
pub struct Logger {
    pub log_path: String,
    pub color_config: ColoredLevelConfig,
}

impl Logger {
    #[must_use]
    pub fn new(log_path: Option<String>, colors: Option<ColoredLevelConfig>) -> Self {
        Self {
            log_path: log_path.unwrap_or_else(|| String::from("/tmp/ruby.log")),
            color_config: colors.unwrap_or_else(|| {
                ColoredLevelConfig::new()
                    .info(Color::Green)
                    .warn(Color::Yellow)
                    .debug(Color::Magenta)
                    .error(Color::Red)
                    .trace(Color::Blue)
            }),
        }
    }

    pub fn log(&self) -> Result<(), InitError> {
        Dispatch::new()
            .level(LevelFilter::Debug)
            .chain({
                let colors = self.color_config;
                let mut logger = Dispatch::new()
                    .format(move |out, message, record| {
                        out.finish(format_args!(
                            "[{} Ruby{}] {} {}",
                            colors.color(record.level()),
                            match (record.file(), record.line()) {
                                (Some(file), Some(line)) => format!(":{}:{}", file, line),
                                _ => "".into(),
                            },
                            Utc::now().format("[%Y-%m-%d %H:%M:%S]"),
                            message
                        ));
                    })
                    .chain(stderr());

                match log_file(&self.log_path) {
                    Ok(log) => logger = logger.chain(log),
                    Err(err) => {
                        eprintln!("Failed to create log file at {}: {}", &self.log_path, err);
                    }
                }

                logger
            })
            .apply()?;

        Ok(())
    }
}
