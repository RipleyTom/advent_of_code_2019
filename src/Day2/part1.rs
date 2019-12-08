use std::fs::File;
use std::io::Read;

pub fn run_puzzle() {
    let mut file = File::open("input_day2.txt").expect("Failed to open input_day2.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let mut vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    vec[1] = 12;
    vec[2] = 2;

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

    println!("Result: {}", vec[0]);
}
