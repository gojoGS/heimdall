use crate::log::LogLine;

type LogFilterCriterion = dyn Fn(&LogLine) -> bool;

pub struct ParseResultFilter {
    criteria: Vec<Box<LogFilterCriterion>>,
}

impl ParseResultFilter {
    pub fn new() -> ParseResultFilter {
        ParseResultFilter { criteria: vec![] }
    }

    pub fn add_criterion(&mut self, criterion: Box<LogFilterCriterion>) -> &mut ParseResultFilter {
        self.criteria.push(criterion);
        self
    }

    // pub fn filter(&self, lines: Vec<LogLine>, crit: Box<LogFilterCriterion>) -> Vec<&LogLine> {
    //     lines.iter().filter(|l| crit(l)).collect::<Vec<_>>()
    // }
}
