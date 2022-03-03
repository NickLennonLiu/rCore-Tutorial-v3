use core::fmt;
use log::{self, info, Level, LevelFilter, Log, Metadata, Record};


struct SimpleLogger;

macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

pub fn print_in_color(args: fmt::Arguments, color_code: u8) {
    crate::console::print(with_color!(args, color_code));
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
    info!("Logger level: {}", log::max_level());
}

impl Log for SimpleLogger{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        print_in_color(
            format_args!(
                "[{:>5}][{}] {}\n",
                record.level(),
                0, // cpu id
                record.args(),
            ),
            level_to_color_code(record.level()),
        );
    }

    fn flush(&self) {

    }
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}