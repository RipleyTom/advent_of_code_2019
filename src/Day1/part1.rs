use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_puzzle() {
    let file = File::open("input_day1.txt").expect("Failed to open input_day1.txt");
    let br = BufReader::new(file);

    let vec: Vec<i64> = br.lines().map(|line| line.expect("Line parsing failed").parse::<i64>().expect("Line => i64 failed")).collect();

    let mut total = 0;
    for v in &vec {
        total += (v / 3) - 2;
    }

    println!("Total : {}", total);
}
