use crate::log::{LogLevel, LogLine};
use crate::parse::{LineParser, ParseResult};

use regex::Regex;

pub struct RegexParser;

impl LineParser for RegexParser {
    fn parse(content: &str) -> ParseResult {
        let lines: Vec<LogLine> = content.split('\n').map(line_to_log_line).collect();

        ParseResult { lines }
    }
}

fn line_to_log_line(line: &str) -> LogLine {
    let re: Regex = Regex::new(r"TRACE|DEBUG|INFO|WARN|ERROR|FATAL").unwrap();

    let captures = re.captures(line);

    match captures {
        None => LogLine {
            line: line.to_string(),
            log_level: LogLevel::None,
        },
        Some(captures) => {
            let captured_string = captures.get(0).unwrap().as_str();
            let log_level = LogLevel::from_str(captured_string);

            LogLine {
                line: line.to_string(),
                log_level,
            }
        }
    }
}
