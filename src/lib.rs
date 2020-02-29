use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sentive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sentive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sentive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sentive {
        search(&config.query, &contents)
    } else {
        search_case_insentive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for i in contents.lines() {
        if i.contains(query) {
            results.push(i);
        }
    }
    results
}
pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for i in contents.lines() {
        if i.to_lowercase().contains(&query) {
            results.push(i);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sentive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insentive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insentive(query, contents)
        );
    }
}
