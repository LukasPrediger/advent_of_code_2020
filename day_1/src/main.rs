use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let file = File::open("input.txt").expect("Error opening file");

    let mut acc = Vec::new();

    let lines = BufReader::new(file).lines();

    for line in lines {
        acc.push(line.unwrap().parse::<u32>().unwrap())
    }

    acc.sort();

    let mut decc = acc.clone();
    decc.sort();

    let mut first_num: &u32 = &0;
    let mut second_num: &u32 = &0;

    for a in acc.iter() {
        for b in decc.iter() {
            if a + b == 2020 {
                first_num = a;
                second_num = b;
                break;
            }
        }
    }
    println!("{} + {} = {}", first_num, second_num, first_num * second_num)
}
