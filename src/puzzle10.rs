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

fn get_params(vec: &Vec<i64>, index: usize, num_params: usize) -> Vec<Param> {
    let mut params = Vec::new();

    for i in 1..num_params+1 {
        params.push(Param::new(vec, index, i));
    }

    params
}

fn op_add(vec: &mut Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 3);

    assert!(params[2].mode == 0);

    vec[params[2].value as usize] = params[0].get_value(vec) + params[1].get_value(vec);

    index + 4
}

fn op_mul(vec: &mut Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 3);

    vec[params[2].value as usize] = params[0].get_value(vec) * params[1].get_value(vec);

    index + 4
}

fn op_input(vec: &mut Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 1);
    assert!(params[0].mode == 0);

    vec[params[0].value as usize] = 5;

    index + 2
}

fn op_output(vec: &Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 1);

    println!("Output command: {}", params[0].get_value(vec));

    index + 2
}

fn op_jump_if_true(vec: &Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 2);

    if params[0].get_value(vec) != 0 {
        params[1].get_value(vec) as usize
    } else {
        index + 3
    }
}

fn op_jump_if_false(vec: &Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 2);

    if params[0].get_value(vec) == 0 {
        params[1].get_value(vec) as usize
    } else {
        index + 3
    }
}

fn op_lessthan(vec: &mut Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 3);

    assert!(params[2].mode == 0);

    let to_store;

    if params[0].get_value(vec) < params[1].get_value(vec) {
        to_store = 1;
    } else {
        to_store = 0;
    }
    vec[params[2].value as usize] = to_store;

    index + 4
}

fn op_equal(vec: &mut Vec<i64>, index: usize) -> usize {
    let params = get_params(vec, index, 3);

    assert!(params[2].mode == 0);

    let to_store;

    if params[0].get_value(vec) == params[1].get_value(vec) {
        to_store = 1;
    } else {
        to_store = 0;
    }
    vec[params[2].value as usize] = to_store;

    index + 4
}


fn run_program(mut vec: Vec<i64>) {
    let mut op_index = 0;

    while op_index < vec.len() {
        match vec[op_index] % 100 {
            1 => op_index = op_add(&mut vec, op_index),
            2 => op_index = op_mul(&mut vec, op_index),
            3 => op_index = op_input(&mut vec, op_index),
            4 => op_index = op_output(&vec, op_index),
            5 => op_index = op_jump_if_true(&vec, op_index),
            6 => op_index = op_jump_if_false(&vec, op_index),
            7 => op_index = op_lessthan(&mut vec, op_index),
            8 => op_index = op_equal(&mut vec, op_index),
            99 => break,
            _ => panic!("Invalid opcode!"),
        }
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_4.txt").expect("Failed to open input_4.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    run_program(vec);
}
