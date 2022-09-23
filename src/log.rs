use std::{env, fmt, io::stderr};

use ::log::LevelFilter;
use chrono::Utc;
use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch, InitError,
};
use uwuifier::{round_up16, uwuify_sse};

#[derive(Default)]
pub struct Logger {
    pub log_path: String,
    pub color_config: ColoredLevelConfig,
    uwu: bool,
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
            uwu: env::var("RUBY_UWU").map(|x| x == "true").unwrap_or(false),
        }
    }

    pub fn log(&self) -> Result<(), InitError> {
        Dispatch::new()
            .level(LevelFilter::Debug)
            .chain({
                let colors = self.color_config;
                let should_uwu = self.uwu;
                let mut logger = Dispatch::new()
                    .format(move |out, message, record| {
                        out.finish(format_args!(
                            "[{} Ruby{}] {} {}",
                            Self::uwu_text(should_uwu, colors.color(record.level())),
                            match (record.file(), record.line()) {
                                (Some(file), Some(line)) => format!(":{}:{}", file, line),
                                _ => "".into(),
                            },
                            Utc::now().format("[%Y-%m-%d %H:%M:%S]"),
                            Self::uwu_text(should_uwu, message)
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

    // there is no reason this should exist but it's hilarious
    fn uwu_text<F: fmt::Display>(should_uwu: bool, text: F) -> String {
        if should_uwu {
            let string = text.to_string();
            let bytes = string.as_bytes();
            let mut temp1 = vec![0u8; round_up16(bytes.len()) * 16];
            let mut temp2 = vec![0u8; round_up16(bytes.len()) * 16];
            String::from_utf8(uwuify_sse(bytes, &mut temp1, &mut temp2).to_vec())
                .expect("Expected a String")
        } else {
            text.to_string()
        }
    }
}
