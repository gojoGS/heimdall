mod dfa_parser;
mod regex_parser;
use crate::log::LogLine;
pub use regex_parser::RegexParser;

pub trait LineParser {
    fn parse(content: &str) -> ParseResult;
}

pub struct ParseResult {
    pub lines: Vec<LogLine>,
}
