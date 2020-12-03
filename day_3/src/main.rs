use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Error opening file");
    let lines = BufReader::new(file).lines();

    let parsed_lines: Vec<Vec<usize>> = lines.map(|line| {
        line.unwrap().chars().map(|char| return if char == '#' { 1usize } else { 0usize }).collect()
    }
    ).collect();

    let slopes: [(usize, usize);5] = [(1,1), (3,1),(5,1), (7,1), (1,2)];

    let result = slopes.iter()
        .map(|(x,y)| check_slope(&x,&y, &parsed_lines))
        .fold(1usize, | curr, next | curr * next);

    println!("{}", result)
}

fn check_slope(slope_x: &usize, slope_y: &usize, lines: &Vec<Vec<usize>>) -> usize {
    let mut x = 0;
    let mut y = 0;

    let max_y = lines.len();
    let max_x = lines.first().unwrap().len();

    let mut result = 0usize;

    while y < max_y {
        let line = &lines[y];

        result += line[x];

        y += slope_y;
        x += slope_x;
        if x >= max_x {
            x = x - max_x
        }
    }
    return result;
}
