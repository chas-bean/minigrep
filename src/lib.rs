use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(s) => s,
            None => return Err("No query string specified"),
        };

        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("No file path specified"),
        };

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
    contents.lines().filter(|l| l.contains(query)).collect()
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
