use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Error opening file");
    let lines = BufReader::new(file).lines();

    let parsed_lines: Vec<Vec<i32>> = lines.map(|line| {
        line.unwrap().chars().map(|char| return if char == '#' { 1 } else { 0 }).collect()
    }
    ).collect();

    let mut x = 0;
    let mut y = 0;

    let max_y = parsed_lines.len();
    let max_x = parsed_lines.first().unwrap().len();

    let mut result = 0;

    while y < max_y {
        let line = &parsed_lines[y];

        result += line[x];

        y += 1;
        x += 3;
        if x >= max_x {
            x = x - max_x
        }
    }

    println!("{}", result);
}
