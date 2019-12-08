use std::fs::File;
use std::io::Read;

fn run_program(mut vec: Vec<i64>, noun: i64, verb: i64) -> i64 {
    vec[1] = noun;
    vec[2] = verb;

    let mut op_index = 0;
    while op_index < vec.len() {
        let op = vec[op_index];

        if op == 99 {
            break;
        }

        let op_1 = vec[vec[op_index + 1] as usize];
        let op_2 = vec[vec[op_index + 2] as usize];
        let op_3 = vec[op_index + 3] as usize;

        match op {
            1 => vec[op_3] = op_1 + op_2,
            2 => vec[op_3] = op_1 * op_2,
            _ => panic!("Invalid opcode!"),
        }

        op_index += 4;
    }

    vec[0]
}

pub fn run_puzzle() {
    let mut file = File::open("input_day2.txt").expect("Failed to open input_day2.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    let mut found = false;
    let mut result = 0;

    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(vec.clone(), noun, verb) == 19690720 {
                result = (noun * 100) + verb;
                found = true;
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }

    if !found {
        println!("Result not found?!");
    } else {
        println!("Result: {}", result);
    }
}
