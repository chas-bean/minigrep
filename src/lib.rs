use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut findings = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            findings.push(line);
        }
    }

    findings
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut findings = vec![];

    for line in contents.lines() {
        if line.contains(&query) {
            findings.push(line);
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "ool";
        let contents = "What a really great\nAnd cool\nProgramming language";

        let result = search(query, contents);

        assert_eq!(result, vec!["And cool"]);
    }

    #[test]
    fn case_insensitive() {
        let query = "COOL";
        let contents = "What a really great\nAnd cool\nProgramming language";

        let result = search_case_insensitive(query, contents);

        assert_eq!(result, vec!["And cool"]);
    }
}
