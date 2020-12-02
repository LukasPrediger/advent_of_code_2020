use std::fs::File;
use std::io::{BufReader, BufRead};

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

        let (low_quantifier, high_quantifier) = (quantifier[0].parse::<usize>().unwrap(), quantifier[1].parse::<usize>().unwrap());

        let occurrences = password.chars().filter(|x| x == &character).count();

        if occurrences >= low_quantifier && occurrences <= high_quantifier {
            correct += 1;
        }
    }
    println!("{}", correct);
}
