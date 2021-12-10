use regex::{Error, Regex};

pub struct MatchConfig {
    match_on: MatchOn,
    patterns: Vec<String>,
    size: usize,
    depth: u8,
}

impl MatchConfig {
    pub fn new(on: MatchOn, patterns: &[&str]) -> Result<MatchConfig, Error> {}
}

pub enum MatchOn {
    Regex,
    Size(usize),
    Depth(u8),
}
