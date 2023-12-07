use std::env;
use std::fs::read_to_string;

pub fn read_file() -> Vec<String> {
    let args : Vec<String> = env::args().collect();

    let mut result = Vec::new();

    for line in read_to_string(&args[1]).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
