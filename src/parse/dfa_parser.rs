use crate::parse::dfa_parser::CharType::{Alpha, Newline, Other};
use crate::parse::dfa_parser::ParserState::{
    EndOfLine, EndOfParse, EndOfWord, FoundLogLevel, InsideWord, Searching,
};

enum ParserState {
    Searching,
    InsideWord,
    EndOfWord,
    FoundLogLevel,
    EndOfLine,
    EndOfParse,
}

impl ParserState {
    pub fn next_state(&self, char_type: &CharType) -> ParserState {
        match self {
            Searching => match char_type {
                Alpha(_) => InsideWord,
                Other(_) => Searching,
                Newline() => EndOfLine,
            },

            InsideWord => match char_type {
                Alpha(_) => InsideWord,
                Other(_) => EndOfWord,
                Newline() => EndOfWord,
            },

            EndOfWord => match char_type {
                Alpha(_) => InsideWord,
                Other(_) => Searching,
                Newline() => EndOfLine,
            },

            FoundLogLevel => match char_type {
                Newline() => EndOfLine,
                _ => FoundLogLevel,
            },

            EndOfLine => Searching,

            EndOfParse => EndOfParse,
        }
    }
}

enum CharType {
    Alpha(char),
    Newline(),
    Other(char),
}

impl CharType {
    pub fn from_char(c: &char) -> CharType {
        match c {
            _ if { c.is_alphabetic() } => Alpha(*c),
            '\n' => Newline(),
            _ => Other(*c),
        }
    }
}
