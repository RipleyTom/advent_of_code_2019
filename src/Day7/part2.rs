use std::collections::{HashSet, VecDeque};
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

struct Program {
    state: Vec<i64>,
    current_op: usize,
    finished: bool,
    input: VecDeque<i64>,
}

impl Program {
    fn new(state: Vec<i64>, input: VecDeque<i64>) -> Program {
        Program {
            state,
            current_op: 0,
            finished: false,
            input,
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn get_params(&self, num_params: usize) -> Vec<Param> {
        let mut params = Vec::new();

        for i in 1..num_params + 1 {
            params.push(Param::new(&self.state, self.current_op, i));
        }

        params
    }

    fn op_add(&mut self) {
        let params = self.get_params(3);
        assert!(params[2].mode == 0);
        self.state[params[2].value as usize] = params[0].get_value(&self.state) + params[1].get_value(&self.state);
        self.current_op += 4;
    }

    fn op_mul(&mut self) {
        let params = self.get_params(3);
        assert!(params[2].mode == 0);
        self.state[params[2].value as usize] = params[0].get_value(&self.state) * params[1].get_value(&self.state);
        self.current_op += 4;
    }

    fn op_input(&mut self) -> bool {
        let params = self.get_params(1);
        assert!(params[0].mode == 0);

        let input = self.input.pop_back();
        if input.is_none() {
            return false;
        }

        self.state[params[0].value as usize] = input.unwrap();
        self.current_op += 2;
        return true;
    }

    fn op_output(&mut self, output: &mut i64) {
        let params = self.get_params(1);
        *output = params[0].get_value(&self.state);
        self.current_op += 2;
    }

    fn op_jump_if_true(&mut self) {
        let params = self.get_params(2);

        if params[0].get_value(&self.state) != 0 {
            self.current_op = params[1].get_value(&self.state) as usize;
        } else {
            self.current_op += 3;
        }
    }

    fn op_jump_if_false(&mut self) {
        let params = self.get_params(2);

        if params[0].get_value(&self.state) == 0 {
            self.current_op = params[1].get_value(&self.state) as usize;
        } else {
            self.current_op += 3;
        }
    }

    fn op_lessthan(&mut self) {
        let params = self.get_params(3);
        assert!(params[2].mode == 0);
        let to_store;
        if params[0].get_value(&self.state) < params[1].get_value(&self.state) {
            to_store = 1;
        } else {
            to_store = 0;
        }
        self.state[params[2].value as usize] = to_store;
        self.current_op += 4;
    }

    fn op_equal(&mut self) {
        let params = self.get_params(3);
        assert!(params[2].mode == 0);
        let to_store;
        if params[0].get_value(&self.state) == params[1].get_value(&self.state) {
            to_store = 1;
        } else {
            to_store = 0;
        }
        self.state[params[2].value as usize] = to_store;
        self.current_op += 4;
    }

    fn run_program(&mut self, input: i64) -> i64 {
        let mut output = 0;

        self.input.push_front(input);

        while self.current_op < self.state.len() {
            match self.state[self.current_op] % 100 {
                1 => self.op_add(),
                2 => self.op_mul(),
                3 => {
                    if !self.op_input() {
                        break;
                    }
                }
                4 => self.op_output(&mut output),
                5 => self.op_jump_if_true(),
                6 => self.op_jump_if_false(),
                7 => self.op_lessthan(),
                8 => self.op_equal(),
                99 => {
                    self.finished = true;
                    break;
                }
                _ => panic!("Invalid opcode!"),
            }
        }

        output
    }
}

fn generate_combination(cur: &mut [i64; 5], index: usize, left: &HashSet<i64>, combinations: &mut Vec<[i64; 5]>) {
    if index == 5 {
        combinations.push(cur.clone());
        return;
    }

    for v in left {
        cur[index] = *v;
        let mut left_clone = left.clone();
        left_clone.remove(v);
        generate_combination(cur, index + 1, &mut left_clone, combinations);
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_day7.txt").expect("Failed to open input_day7.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    let mut combinations = Vec::new();
    let mut array = [0; 5];
    let left = [5, 6, 7, 8, 9].iter().cloned().collect();
    generate_combination(&mut array, 0, &left, &mut combinations);

    let mut max = 0;

    for c in &combinations {
        let mut output = 0;
        let mut programs = Vec::new();
        for i in 0..5 {
            let mut input = VecDeque::new();
            input.push_back(c[i]);
            programs.push(Program::new(vec.clone(), input))
        }

        while !programs[4].is_finished() {
            for i in 0..5 {
                output = programs[i].run_program(output);
            }
        }

        if output > max {
            max = output;
        }
    }

    println!("Max is: {}", max);
}
