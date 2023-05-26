use crate::log::LogLine;

pub struct ParseResultFilter {
    filters: Vec<Box<LogFilter>>,
}

type LogFilter = dyn Fn(&LogLine) -> bool;

impl ParseResultFilter {
    pub fn new() -> ParseResultFilter {
        ParseResultFilter { filters: vec![] }
    }

    pub fn add_filter(&mut self, filter: Box<LogFilter>) -> &mut ParseResultFilter {
        self.filters.push(filter);
        self
    }

    pub fn filter(&self, vec: &Vec<LogLine>) {
        for elem in vec {
            println!("{}", self.matches(elem))
        }
    }

    fn matches(&self, line: &LogLine) -> bool {
        for filter in &self.filters {
            if !filter(line) {
                return false;
            }
        }

        true
    }
}
