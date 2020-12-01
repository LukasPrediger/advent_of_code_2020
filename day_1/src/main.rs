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

    let mut decc = acc.clone();
    decc.reverse();

    let mut first_num: &u32 = &0;
    let mut second_num: &u32 = &0;
    let mut third_num: &u32 = &0;

    for a in acc.iter() {
        for b in decc.iter() {
            for c in acc.iter() {
                if a + b + c == 2020 {
                    first_num = a;
                    second_num = b;
                    third_num = c;
                    break;
                }
            }
        }
    }
    println!("{} * {} * {}= {}", first_num, second_num, third_num, first_num * second_num * third_num);
}
