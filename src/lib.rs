use std::{error::Error, fs};

use colored::Colorize;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
            case_sensitive: false,
        })
    }
}

#[derive(Debug)]
pub struct LineMatch {
    pub start: usize,
    pub end: usize,
    pub case_sensitive: bool,
    pub query: String,
}

impl LineMatch {
    pub fn new(start: usize, end: usize, case_sensitive: bool, query: String) -> LineMatch {
        LineMatch {
            start,
            end,
            case_sensitive,
            query,
        }
    }

    pub fn get_value_to_highlight(&self, value: &str) -> String {
        let mut result = String::new();

        let mut indexes_with_diferent_case: Vec<usize> = Vec::new();

        for char in self.query.chars() {
            let index = value[self.start..self.end].find(char);

            if let Some(index) = index {
                indexes_with_diferent_case.push(index);
            }
        }

        let mut last_index = 0;
        for index in indexes_with_diferent_case {
            result.push_str(
                &value[self.start + last_index..self.start + index]
                    .to_string()
                    .yellow()
                    .to_string(),
            );
            result.push_str(
                &value[self.start + index..self.start + index + 1]
                    .to_string()
                    .green()
                    .to_string(),
            );
            last_index = index + 1;
        }

        result.push_str(&value[self.start + last_index..self.end]);

        result
    }
}

pub struct LineMatchCollection<'a> {
    pub matchs: Vec<LineMatch>,
    pub line_content: &'a str,
    pub line_index: usize,
}

impl LineMatchCollection<'_> {
    pub fn new<'a>(self, line_index: usize, line_content: &'a str) -> LineMatchCollection<'a> {
        LineMatchCollection { matchs: Vec::new(), line_index, line_content }
    }

    pub fn add_match(&mut self, line_match: LineMatch) {
        self.matchs.push(line_match);
    }
}

#[derive(Debug)]
pub struct Line {
    pub matchs: Vec<LineMatch>,
    pub line_index: usize,
}

impl Line {
    pub fn new(line_index: usize) -> Line {
        Line {
            matchs: Vec::new(),
            line_index,
        }
    }

    pub fn add_match(&mut self, line_match: LineMatch) {
        self.matchs.push(line_match);
    }
}

pub fn search_in_line(query: &str, line: &str) -> Vec<LineMatch> {
    let mut indexes: Vec<LineMatch> = Vec::new();

    let query_lower = query.to_lowercase();
    let line_lower = line.to_lowercase();

    for (index, _) in line_lower.match_indices(&query_lower.as_str()) {
        let start_index = index;
        let end_index = index + query.len();

        let value = &line[start_index..end_index];
        let case_sensitive = value == query;

        let line_match = LineMatch::new(start_index, end_index, case_sensitive, query.to_string());

        indexes.push(line_match);
    }

    indexes
}

pub fn print_highlighted_results(highlighted_indexes: Line, line: &str) {
    let matchs = highlighted_indexes.matchs;
    let mut result = String::new();

    let mut last_end = 0;

    for match_index in matchs {
        let value: String = match_index.get_value_to_highlight(line);

        result.push_str(&line[last_end..match_index.start]);
        result.push_str(value.as_str());

        last_end = match_index.end;
    }

    result.push_str(&line[last_end..]);

    println!("{}", result);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.filename)?;

    let mut lines_count = 0;
    let mut lines_match_collections: Vec<LineMatchCollection> = Vec::new();

    for line in contents.lines() {
        let matchs = search_in_line(&config.query, line);

        if matchs.len() > 0 {
            lines_match_collections.push(LineMatchCollection { matchs, line_index: lines_count, line_content: line });
        }

        lines_count += 1;
    }

    for line_match_colection in lines_match_collections {
        print_highlighted_results(Line { matchs: line_match_colection.matchs, line_index: line_match_colection.line_index }, line_match_colection.line_content);
    }

    Ok(())
}
