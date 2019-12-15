use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::cmp;

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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Unexplored = 0,
    Empty = 1,
    InitialPos = 2,
    OxygenSys = 3,
    Wall = 4,
}

fn get_dir_command(dir_x: i64, dir_y: i64) -> i64 {
    if dir_x != 0 {
        if dir_x < 0 {
            3
        } else {
            4
        }
    } else {
        if dir_y < 0 {
            1
        } else {
            2
        }
    }
}

fn print_map(map: &[[(TileType, u64); 500]; 500]) {
    let (mut min_x, mut min_y) = (std::usize::MAX, std::usize::MAX);
    let (mut max_x, mut max_y) = (0, 0);

    println!("Map:");

    for y in 0..500 {
        for x in 0..500 {
            if map[y][x].0 != TileType::Unexplored {
                min_x = cmp::min(x, min_x);
                min_y = cmp::min(y, min_y);
                max_x = cmp::max(x, max_x);
                max_y = cmp::max(y, max_y);
            }
        }
    }

    let width = (max_x - min_x) + 1;

    for y in min_y..max_y + 1 {
        let mut line = vec![' '; width];

        for x in min_x..max_x + 1 {
            let dachar;
            match map[y][x].0 {
                TileType::Unexplored => dachar = ' ',
                TileType::Empty => dachar = '.',
                TileType::InitialPos => dachar = 'R',
                TileType::OxygenSys => dachar = 'O',
                TileType::Wall => dachar = '#',
            }
            line[x - min_x] = dachar;
        }

        let string: String = line.iter().collect();

        println!("{}", string);
    }
}

fn flood_fill(program: &mut Program, map: &mut [[(TileType, u64); 500]; 500], x: i64, y: i64, dir_x: i64, dir_y: i64) {
    if map[(y + dir_y) as usize][(x + dir_x) as usize].0 == TileType::Unexplored {
        let command = get_dir_command(dir_x, dir_y);

        let res = program.run_program(command);
        let res = res[res.len()-1];
        let ret_tile;
        match res {
            0 => ret_tile = TileType::Wall,
            1 => ret_tile = TileType::Empty,
            2 => ret_tile = TileType::OxygenSys,
            _ => unreachable!(),
        }

        map[(y + dir_y) as usize][(x + dir_x) as usize].0 = ret_tile;
        // print_map(map);

        if ret_tile != TileType::Wall {
            flood_fill(program, map, x + dir_x, y + dir_y, -1, 0);
            flood_fill(program, map, x + dir_x, y + dir_y, 1, 0);
            flood_fill(program, map, x + dir_x, y + dir_y, 0, -1);
            flood_fill(program, map, x + dir_x, y + dir_y, 0, 1);

            let command = get_dir_command(-dir_x, -dir_y);
            program.run_program(command);
        }
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_day15.txt").expect("Failed to open input_day15.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    let mut program = Program::new(&vec, VecDeque::new());

    let mut map = [[(TileType::Unexplored, std::u64::MAX - 1); 500]; 500];
    let (x, y) = (250, 250);

    map[y as usize][x as usize] = (TileType::InitialPos, std::u64::MAX - 1);

    flood_fill(&mut program, &mut map, x, y, -1, 0);
    flood_fill(&mut program, &mut map, x, y, 1, 0);
    flood_fill(&mut program, &mut map, x, y, 0, -1);
    flood_fill(&mut program, &mut map, x, y, 0, 1);

    'search: for y in 0..500 {
        for x in 0..500 {
            if map[y][x].0 == TileType::OxygenSys {
                map[y][x].1 = 0;
                break 'search;
            }
        }
    }

    let mut changed = true;

    while changed {
        changed = false;

        for y in 0..500 {
            for x in 0..500 {
                if map[y][x].0 == TileType::Empty || map[y][x].0 == TileType::InitialPos {
                    let mut min_steps = std::u64::MAX - 1;
                    if map[y][x-1].1 < min_steps { min_steps = map[y][x-1].1; }
                    if map[y][x+1].1 < min_steps { min_steps = map[y][x+1].1; }
                    if map[y-1][x].1 < min_steps { min_steps = map[y-1][x].1; }
                    if map[y+1][x].1 < min_steps { min_steps = map[y+1][x].1; }

                    min_steps += 1;

                    if min_steps < map[y][x].1 {
                        map[y][x].1 = min_steps;
                        changed = true;
                    }
                }
            }
        }
    }

    let mut max = 0;

    for y in 0..500 {
        for x in 0..500 {
            if map[y][x].0 == TileType::Empty || map[y][x].0 == TileType::InitialPos {
                if map[y][x].1 > max {
                    max = map[y][x].1;
                }
            }
        }
    }

    println!("Last tile gets oxygen after {} minutes", max);

}
