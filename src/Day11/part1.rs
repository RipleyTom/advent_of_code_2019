use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

#[repr(u8)]
#[derive(PartialEq)]
enum ParamType {
    PositionMode = 0,
    ImmediateMode = 1,
    RelativeMode = 2,
}

fn get_param_type(mode: i64) -> ParamType {
    match mode {
        0 => ParamType::PositionMode,
        1 => ParamType::ImmediateMode,
        2 => ParamType::RelativeMode,
        u => panic!("Unexpected parameter type: {}", u),
    }
}

struct Param {
    mode: ParamType,
    value: i64,
    relative_base: i64,
}

impl Param {
    fn new(vec: &Vec<i64>, index: usize, param_index: usize, relative_base: i64) -> Param {
        let mode = get_param_type((vec[index] / 10i64.pow((param_index + 1) as u32)) % 10);
        let value = vec[index + param_index];
        Param { mode, value, relative_base }
    }

    fn get_value(&self, vec: &Vec<i64>) -> i64 {
        match self.mode {
            ParamType::PositionMode => vec[self.value as usize],
            ParamType::ImmediateMode => self.value,
            ParamType::RelativeMode => vec[(self.value + self.relative_base) as usize],
        }
    }

    fn set_value(&self, vec: &mut Vec<i64>, value: i64) {
        match self.mode {
            ParamType::PositionMode => vec[self.value as usize] = value,
            ParamType::ImmediateMode => panic!("set_value called with a parameter in ImmediateMode!"),
            ParamType::RelativeMode => vec[(self.value + self.relative_base) as usize] = value,
        }
    }
}

struct Program {
    state: Vec<i64>,
    current_op: usize,
    finished: bool,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative_base: i64,
}

impl Program {
    fn new(program: &Vec<i64>, input: VecDeque<i64>) -> Program {
        let mut state = vec![0; 10000];
        state.as_mut_slice()[0..program.len()].copy_from_slice(program.as_slice());

        Program {
            state,
            current_op: 0,
            finished: false,
            input,
            output: VecDeque::new(),
            relative_base: 0,
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn get_params(&self, num_params: usize) -> Vec<Param> {
        let mut params = Vec::new();

        for i in 1..num_params + 1 {
            params.push(Param::new(&self.state, self.current_op, i, self.relative_base));
        }

        params
    }

    fn op_add(&mut self) {
        let params = self.get_params(3);

        let sum = params[0].get_value(&self.state) + params[1].get_value(&self.state);
        params[2].set_value(&mut self.state, sum);

        self.current_op += 4;
    }

    fn op_mul(&mut self) {
        let params = self.get_params(3);

        let product = params[0].get_value(&self.state) * params[1].get_value(&self.state);
        params[2].set_value(&mut self.state, product);

        self.current_op += 4;
    }

    fn op_input(&mut self) -> bool {
        let params = self.get_params(1);

        let input = self.input.pop_back();
        if input.is_none() {
            return false;
        }

        params[0].set_value(&mut self.state, input.unwrap());
        self.current_op += 2;
        return true;
    }

    fn op_output(&mut self) {
        let params = self.get_params(1);
        let value = params[0].get_value(&self.state);
        self.output.push_back(value);
        self.current_op += 2;

        //println!("Output: {}", value);
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

        let to_store;
        if params[0].get_value(&self.state) < params[1].get_value(&self.state) {
            to_store = 1;
        } else {
            to_store = 0;
        }
        params[2].set_value(&mut self.state, to_store);
        self.current_op += 4;
    }

    fn op_equal(&mut self) {
        let params = self.get_params(3);

        let to_store;
        if params[0].get_value(&self.state) == params[1].get_value(&self.state) {
            to_store = 1;
        } else {
            to_store = 0;
        }
        params[2].set_value(&mut self.state, to_store);
        self.current_op += 4;
    }

    fn op_adjust_relative_base(&mut self) {
        let params = self.get_params(1);

        self.relative_base += params[0].get_value(&self.state);
        self.current_op += 2;
    }

    fn run_program(&mut self, input: i64) -> &VecDeque<i64> {
        self.output.clear();

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
                4 => self.op_output(),
                5 => self.op_jump_if_true(),
                6 => self.op_jump_if_false(),
                7 => self.op_lessthan(),
                8 => self.op_equal(),
                9 => self.op_adjust_relative_base(),
                99 => {
                    self.finished = true;
                    break;
                }
                _ => panic!("Invalid opcode!"),
            }
        }

        &self.output
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_day11.txt").expect("Failed to open input_day11.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    let mut panels = [[(0u8, false); 1000]; 1000];
    let (mut cur_x, mut cur_y) = (500, 500);
    let mut dir = 0;

    let mut program = Program::new(&vec, VecDeque::new());

    let mut color_mode = true;

    while !program.is_finished() {
        let output = program.run_program(panels[cur_y][cur_x].0 as i64);

        for o in output {
            if color_mode {
                panels[cur_y][cur_x] = (*o as u8, true);
                color_mode = false;
            } else {
                match *o {
                    0 => dir -= 1,
                    1 => dir += 1,
                    _ => panic!("Direction not 0 or 1"),
                }

                let mut cur_dir = dir % 4;
                if cur_dir < 0 {
                    cur_dir = 4 + cur_dir;
                }

                match cur_dir {
                    0 => cur_y -= 1,
                    1 => cur_x += 1,
                    2 => cur_y += 1,
                    3 => cur_x -= 1,
                    _ => unreachable!(),
                }
                color_mode = true;
            }
        }
    }

    let mut num_painted = 0;

    for y in 0..1000 {
        for x in 0..1000 {
            if panels[y][x].1 {
                num_painted += 1;
            }
        }
    }

    println!("Num of tiles painted: {}", num_painted);
}
