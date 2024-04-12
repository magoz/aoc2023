use std::fs;

fn line_number(line: &str) -> i32 {
    let numbers = line
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>();

    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();

    let number: i32 = format!("{}{}", first, last).parse().unwrap();
    return number;
}

fn main() {
    let input = fs::read_to_string("src/input/p1").expect("Failed to read input file");
    let sum: i32 = input.lines().map(line_number).sum();
    println!("{}", sum);
}
