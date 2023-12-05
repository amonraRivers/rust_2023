use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(filename: &str) -> Vec<String> {
    let input: Vec<String>;
    let file = File::open(String::from("../../2023/") + filename);
    if let Ok(f) = file {
        let lines = BufReader::new(f).lines();
        input = lines.map(|x| x.unwrap()).collect();
    } else {
        input = Vec::new();
    }
    input
}
