use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let file = fs::read_to_string(filename).expect("File should be readable");

    let mut findings = vec![];
    for line in file.lines() {
        if line.contains(query) {
            findings.push(line);
        }
    }

    println!("Found:\n{:?}", findings);
}
