use std::fs::File;
use std::io::Read;

struct Param {
    mode: u8,
    value: i64,
}

impl Param {
    fn new(vec: &Vec<i64>, index: usize, param_index: usize) -> Param {
        let mode = ((vec[index] / 10i64.pow((param_index + 1) as u32)) % 10) as u8;
        let value = vec[index + param_index];

        assert!(mode == 0 || mode == 1);

        Param { mode, value }
    }

    fn get_value(&self, vec: &Vec<i64>) -> i64 {
        match self.mode {
            0 => vec[self.value as usize],
            1 => self.value,
            u => panic!("Unimplemented mode: {}", u),
        }
    }
}

fn op_add(vec: &mut Vec<i64>, index: usize) -> i64 {
    let param1 = Param::new(vec, index, 1);
    let param2 = Param::new(vec, index, 2);
    let param3 = Param::new(vec, index, 3);

    assert!(param3.mode == 0);

    vec[param3.value as usize] = param1.get_value(vec) + param2.get_value(vec);

    4
}

fn op_mul(vec: &mut Vec<i64>, index: usize) -> i64 {
    let param1 = Param::new(vec, index, 1);
    let param2 = Param::new(vec, index, 2);
    let param3 = Param::new(vec, index, 3);

    vec[param3.value as usize] = param1.get_value(vec) * param2.get_value(vec);

    4
}

fn input(vec: &mut Vec<i64>, index: usize) -> i64 {
    let param1 = Param::new(vec, index, 1);
    assert!(param1.mode == 0);

    vec[param1.value as usize] = 1;

    2
}

fn output(vec: &Vec<i64>, index: usize) -> i64 {
    let param1 = Param::new(vec, index, 1);

    println!("Output command: {}", param1.get_value(vec));

    2
}


fn run_program(mut vec: Vec<i64>) {
    let mut op_index = 0;

    while op_index < vec.len() {
        let ret;
        match vec[op_index] % 100 {
            1 => ret = op_add(&mut vec, op_index),
            2 => ret = op_mul(&mut vec, op_index),
            3 => ret = input(&mut vec, op_index),
            4 => ret = output(&vec, op_index),
            99 => break,
            _ => panic!("Invalid opcode!"),
        }
        op_index = (op_index as i64 + ret) as usize;
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_day5.txt").expect("Failed to open input_day5.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    run_program(vec);
}
