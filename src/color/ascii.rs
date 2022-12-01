use crate::log::{LogLevel, LogLine};
use colored::{Color, ColoredString, Colorize};

pub fn colorize_log_line(log_line: LogLine) -> ColoredString {
    let log_level = log_line.log_level;
    let line = log_line.line;

    match log_level {
        LogLevel::None => line.italic(),
        LogLevel::Trace => line.color(Color::TrueColor {
            r: 105,
            g: 105,
            b: 105,
        }),
        LogLevel::Debug => line.color(Color::White),
        LogLevel::Info => line.bold().color(Color::Blue),
        LogLevel::Warn => line.bold().color(Color::Yellow),
        LogLevel::Error => line.bold().color(Color::Red),
        LogLevel::Fatal => line.bold().color(Color::Black).on_color(Color::Red),
    }
}
