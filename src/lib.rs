use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_path)?;

    let mut findings = vec![];
    for line in file.lines() {
        if line.contains(&config.query) {
            findings.push(line);
        }
    }

    println!("Found:\n{:?}", findings);

    Ok(())
}
