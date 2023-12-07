mod input;

use self::input::read_file;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let lines : Vec<String> = read_file(&args[1]);

    let mut sum : u32 = 0;

    for line in lines {
        sum += extract_value(&line)
    }

    println!("{}", sum)
}

fn extract_value(input: &str) -> u32 {
    let digits : String = replace_word_digits(input).chars().filter(|c| c.is_digit(10)).collect();

    let first = digits.chars().nth(0).unwrap().to_string();
    let last = digits.chars().nth(digits.len() - 1).unwrap().to_string();


    (first + &last).parse::<u32>().unwrap()
}

fn replace_word_digits(input: &str) -> String {
    const NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut replaced = String::from(input);

    for (i, num_str) in (*NUMBERS).iter().enumerate() {
        let num = (i + 1).to_string();
        replaced = replaced.replace(&num_str.to_string(), &num);
    }

    replaced.to_string()
}
