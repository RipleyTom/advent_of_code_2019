use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_puzzle() {
    let file = File::open("input_1.txt").expect("Failed to open input_1.txt");
    let br = BufReader::new(file);

    let vec: Vec<i64> = br.lines().map(|line| line.expect("Line parsing failed").parse::<i64>().expect("Line => i64 failed")).collect();

    let mut total_fuel = 0;
    for v in &vec {
        let mut fuel_input = v.clone();
        loop {
            fuel_input = if (fuel_input / 3) - 2 < 0 { 0 } else { (fuel_input / 3) - 2 };
            if fuel_input == 0 {
                break;
            }
            total_fuel += fuel_input;
        }
    }

    println!("Puzzle 2 total : {}", total_fuel);
}
