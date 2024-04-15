use std::io;
use std::io::Write;

pub fn input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_owned();
}

pub fn input_float() -> f32 {
    let input: Result<f32, _> = input().parse();
    return input.expect("Error parsing float");
}

pub fn input_char() -> char {
    let input: Result<char, _> = input().parse();
    return input.expect("Error parsing char");
}