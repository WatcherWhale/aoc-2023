mod input;

use self::input::read_file;

fn main() {
    let lines : Vec<String> = read_file("input.txt");

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
