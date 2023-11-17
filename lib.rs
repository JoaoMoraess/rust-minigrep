use colored::Colorize;
use std::{error::Error, fs};

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
pub struct HighlightStringIndex {
    pub start: usize,
    pub end: usize,
    pub case_sensitive: bool,
    pub line_index: usize,
}

impl HighlightStringIndex {
    pub fn new(start: usize, end: usize, case_sensitive: bool, line_index: usize) -> HighlightStringIndex {
        HighlightStringIndex {
            start,
            end,
            case_sensitive,
            line_index
        }
    }
}

pub struct SearchResult {
    pub value: String,
    pub line_index: usize,
}



pub fn run(config: Config) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.filename)?;

    Ok(search_with_case_insensitive(&config.query, &contents))
}

pub fn search(query: &str, contents: &str) -> Vec<SearchResult> {
    let mut results: Vec<SearchResult> = Vec::new();
    let lines: std::str::Lines<'_> = contents.lines();

    let mut line_index: usize = 0;
    for line in lines {
        if line.contains(&query) {
            let result = SearchResult {
                value: line.to_string(),
                line_index,
            };

            results.push(result);
        }
        line_index += 1;
    }

    results
}

pub fn search_with_case_insensitive(query: &str, contents: &str) -> Vec<SearchResult> {
    let mut results: Vec<SearchResult> = Vec::new();
    let lines: std::str::Lines<'_> = contents.lines();

    let mut line_index: usize = 0;
    for line in lines {
        let lower_line = line.to_lowercase();
        let lower_query = query.to_lowercase();

        if lower_line.contains(&lower_query) {
            results.push(SearchResult {
                value: line.to_string(),
                line_index,
            });
        }

        line_index += 1;
    }
    results
}

pub fn print_results(results: Vec<String>) {
    for result in results {
        println!("{}", result);
    }
}

pub fn get_highlighted_string_indexes(query: &str, contents: &str, content_string_index: usize) -> Vec<HighlightStringIndex> {
    let mut indexes = Vec::new();

    let lower_query = query.to_lowercase();
    let lower_contents = contents.to_lowercase();

    let match_indexes: std::str::MatchIndices<'_, &str> = lower_contents.match_indices(&lower_query);

    for match_index in match_indexes {
        let start = match_index.0;
        let end = match_index.0 + query.len();

        let is_equal_to_query = &contents[start..end] == query;
        let highlight_string_index = HighlightStringIndex::new(start, end, is_equal_to_query, content_string_index);

        indexes.push(highlight_string_index);
    }

    indexes
}

pub fn get_highligted_results(highligtet_indexes: HighlightStringIndexCollection, contents: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for index in highligtet_indexes.indexes {
        let start = index.start;
        let end = index.end;
        let line_index = index.line_index;
        let case_sensitive = index.case_sensitive;

        let line = contents.lines().nth(line_index).unwrap();

        let mut result = String::new();

        if case_sensitive {
            result.push_str(&line[..start]);
            result.push_str(&line[start..end].green().to_string());
            result.push_str(&line[end..]);
        } else {
            result.push_str(&line[..start]);
            result.push_str(&line[start..end].to_string());
            result.push_str(&line[end..]);
        }

        results.push(result);
    }


    results
}

// #[cfg(test)]
// mod search {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }

//     #[test]
//     fn two_results() {
//         let query = "t";
//         let contents = "\nRust:\nsafe, fast, productive.\nPick hree.\nDuc ape.";

//         assert_eq!(
//             vec!["Rust:", "safe, fast, productive."],
//             search(query, contents)
//         );
//     }

//     #[test]
//     fn case_sensitive() {
//         let query = "rust";
//         let contents = "\nRust:\nNot Rust";
//         let expected: Vec<String> = vec![];

//         assert_eq!(expected, search(query, contents))
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rust";
//         let contents = "\nRust:\nNot Rust";
//         let expected: Vec<String> = vec![String::from("Rust:"), String::from("Not Rust")];

//         assert_eq!(expected, search_with_case_insensitive(query, contents))
//     }
// }

// #[cfg(test)]
// mod highligt {
//     use super::*;

//     #[test]
//     fn empty_results() {
//         let contents = "t";
//         let results: Vec<HighlightStringIndex> = Vec::new();
//         let expected: Vec<String> = vec![];

//         assert_eq!(expected, get_highligted_results(results, contents));
//     }

//     #[test]
//     fn highligted_results() {
//         let contents = "Te amo Ariane";
//         let results: Vec<HighlightStringIndex> = vec![HighlightStringIndex::new(0, 2, true, 0)];
//         let expected = vec![String::from("Te".green().to_string() + " amo Ariane")];

//         assert_eq!(expected, get_highligted_results(results, contents));
//     }
// }

// #[cfg(test)]
// mod get_highligted_results_indexes {
//     use super::*;

//     #[test]
//     fn results_indexes() {
//         let query = "Rust";
//         let contents = "\nRust: rust";

//         let expected = vec![HighlightStringIndex::new(1, 3, true, 0), HighlightStringIndex::new(13, 16, false, 0)];
        
//         let result = get_highlighted_string_indexes(query, contents, 0);

//         assert_eq!(expected[0].start, result[0].start);
//         assert_eq!(expected[0].case_sensitive, result[0].case_sensitive);

//         assert_eq!(expected[1].start, result[1].start);
//         assert_eq!(expected[1].case_sensitive, result[1].case_sensitive);
//     }

//     #[test]
//     fn empty_results() {
//         let query = "TESTESTES";
//         let contents = "\nRust:\nsafe, rust, productive.\nPick hree.\nDuc ape.";
//         let result = get_highlighted_string_indexes(query, contents, 0);

//         assert!(result.is_empty());
//     }
// }
