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

    fn _is_finished(&self) -> bool {
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

        let input = self.input.pop_front();
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

        // println!("Output: {}", value);
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

    fn run_program(&mut self, input: Vec<i64>) -> &VecDeque<i64> {
        self.output.clear();

        self.input.extend(input);

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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Empty = 0,
    Scaffold = 1,
}

pub fn run_puzzle() {
    let mut file = File::open("input_day17.txt").expect("Failed to open input_day17.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    let mut program = Program::new(&vec, VecDeque::new());

    let map_output = program.run_program(Vec::new());

    let map_string: String = map_output.iter().map(|v| (*v as u8) as char).collect();

    let mut map_vec: Vec<Vec<TileType>> = Vec::new();

    for l in map_string.trim().lines() {
        let mut new_line = Vec::new();
        for c in l.chars() {
            let tile = match c {
                '.' => TileType::Empty,
                '#' | '^' | '<' | '>' | 'v' => TileType::Scaffold,
                o => panic!("Unhandled Tile: {}", o),
            };
            new_line.push(tile);
        }
        map_vec.push(new_line);
    }

    let height = map_vec.len();
    let width = map_vec[0].len();

    let mut sum = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if map_vec[y][x] == TileType::Scaffold {
                if map_vec[y - 1][x] == TileType::Scaffold && map_vec[y + 1][x] == TileType::Scaffold && map_vec[y][x - 1] == TileType::Scaffold && map_vec[y][x + 1] == TileType::Scaffold {
                    // Intersection
                    sum += x * y;
                }
            }
        }
    }

    println!("Sum is {}", sum);
}
