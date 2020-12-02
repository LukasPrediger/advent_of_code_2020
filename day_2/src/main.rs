use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Error opening file");
    let lines = BufReader::new(file).lines();

    let mut correct = 0;

    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(":").collect();
        let (constraint, password) = (parts[0].trim(), parts[1].trim());

        let constraint_parts: Vec<&str> = constraint.split_whitespace().collect();
        let quantifier: Vec<&str> = constraint_parts[0].split("-").collect();
        let character = constraint_parts[1].chars().next().unwrap();

        let (first_pos, second_pos) = (quantifier[0].parse::<usize>().unwrap(), quantifier[1].parse::<usize>().unwrap());

        let password_chars: Vec<char> = password.chars().collect();
        let first_char: &char = password_chars.get(first_pos-1).unwrap_or(&'-');
        let second_char: &char = password_chars.get(second_pos-1).unwrap_or(&'-');

        if (first_char == &character) ^ (second_char == &character) {
            correct += 1;
        }
    }
    println!("{}", correct);
}
