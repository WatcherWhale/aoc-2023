mod input;

use std::env;
use self::input::read_file;

extern crate sorted_list;
use sorted_list::SortedList;

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
    let digits : String = find_digits(input);

    let first = digits.chars().nth(0).unwrap().to_string();
    let last = digits.chars().nth(digits.len() - 1).unwrap().to_string();

    (first + &last).parse::<u32>().unwrap()
}

fn find_digits(input: &str) -> String {
    const NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut list : SortedList<i32, String> = SortedList::new();

    for (i, num_str) in (*NUMBERS).iter().enumerate() {
        let indices : Vec<_> = input.match_indices(num_str).map(|(i, _)|i).collect();
        for index in indices {
            list.insert(index as i32, ( (i % 9) + 1).to_string());
        }
    }

    let mut result : String = String::new();
    for (_, kv) in list.iter().enumerate() {
        result.push_str(kv.1);
    }

    result
}
