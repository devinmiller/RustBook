use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new<I>(mut args: I) -> Result<Config, &'static str>
        where I: Iterator<Item=String> {
        // skip executable name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results  {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
} 

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_query_argument() {
        let args = vec![
            String::from("file/name/here.exe")
        ];

        let config = Config::new(args.into_iter());

        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("Didn\'t get a query string"));
    }

    #[test]
    fn no_file_name_argument() {
        let args = vec![
            String::from("file/name/here.exe"),
            String::from("emperor"), 

        ];

        let config = Config::new(args.into_iter());

        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("Didn\'t get a file name"));
    }

    #[test]
    fn with_arguments() {
        let args = vec![
            String::from("test.exe"),
            String::from("emperor"), 
            String::from("warhammer.txt")
        ];

        let config = Config::new(args.into_iter())
            .expect("Error creating Config for test");

        assert_eq!(config.query, "emperor");
        assert_eq!(config.filename, "warhammer.txt");
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

        #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}