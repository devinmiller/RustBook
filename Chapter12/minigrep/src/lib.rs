use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arguments() {
        let args = Vec::new();
        let config = Config::new(&args);

        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("not enough arguments"));
    }

    #[test]
    fn with_arguments() {
        let args = vec![
            String::from("test.exe"),
            String::from("emperor"), 
            String::from("warhammer.txt")
        ];

        let config = Config::new(&args)
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
}