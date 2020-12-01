use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Error opening file");

    let mut acc = Vec::new();

    let lines = BufReader::new(file).lines();

    for line in lines {
        acc.push(line.unwrap().parse::<u32>().unwrap())
    }
    acc.sort();

    'outer: for a in acc.iter() {
        for b in acc.iter() {
            for c in acc.iter() {
                if a + b + c == 2020 {
                    let first_num = a;
                    let second_num = b;
                    let third_num = c;
                    println!("{} * {} * {}= {}", first_num, second_num, third_num, first_num * second_num * third_num);
                    break 'outer;
                }
            }
        }
    }
}
