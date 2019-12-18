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

#[derive(Clone)]
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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
impl Direction {
    fn get_dir(dir: u8) -> Direction {
        match dir {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Unsupported direction"),
        }
    }
    fn get_next(&self) -> Direction {
        let mut dir = *self as u8;
        if dir == Direction::West as u8 {
            dir = Direction::North as u8
        } else {
            dir += 1;
        }
        Direction::get_dir(dir)
    }
    fn get_prev(&self) -> Direction {
        let mut dir = *self as u8;
        if dir == Direction::North as u8 {
            dir = Direction::West as u8
        } else {
            dir -= 1;
        }
        Direction::get_dir(dir)
    }
    fn get_x(&self) -> i64 {
        match *self {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0,
        }
    }
    fn get_y(&self) -> i64 {
        match *self {
            Direction::North => -1,
            Direction::South => 1,
            _ => 0,
        }
    }
}

struct Map {
    map: Vec<Vec<TileType>>,
    width: i64,
    height: i64,
}
impl Map {
    fn new(map: Vec<Vec<TileType>>) -> Map {
        let height = map.len() as i64;
        let width = map[0].len() as i64;
        Map { map, width, height }
    }
    fn is_scaffold(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }

        if self.map[y as usize][x as usize] == TileType::Scaffold {
            true
        } else {
            false
        }
    }
}

struct Robot {
    x: i64,
    y: i64,
    dir: Direction,
}
impl Robot {
    fn new(x: i64, y: i64, dir: Direction) -> Robot {
        Robot { x, y, dir }
    }
}

struct Instruction {
    dir_char: char,
    steps: i64,
}
impl Instruction {
    fn new(dir_char: char, steps: i64) -> Instruction {
        Instruction {
            dir_char,
            steps,
        }
    }
    fn to_string(&self) -> String {
        let mut res_str = String::new();
        res_str.push(self.dir_char);
        res_str += ",";
        res_str += &self.steps.to_string();
        res_str += ",";

        res_str
    }
}

fn test_dir(map: &Map, robot: &Robot, dir: Direction) -> i64 {
    let mut num_scaffold = 0;
    let mut x = robot.x + dir.get_x();
    let mut y = robot.y + dir.get_y();
    while map.is_scaffold(x, y) {
        num_scaffold += 1;
        x += dir.get_x();
        y += dir.get_y();
    }

    num_scaffold
}

fn update_robot(robot: &mut Robot, dir_char: char, steps: i64, dir: Direction) -> Instruction {
    robot.x += dir.get_x() * steps;
    robot.y += dir.get_y() * steps;
    robot.dir = dir;

    Instruction::new(dir_char, steps)
}

fn get_next_instruction(map: &Map, robot: &mut Robot) -> Option<Instruction> {
    let num_next = test_dir(map, robot, robot.dir.get_next());
    if num_next != 0 {
        return Some(update_robot(robot, 'R', num_next, robot.dir.get_next()));
    }

    let num_prev = test_dir(map, robot, robot.dir.get_prev());
    if num_prev != 0 {
        return Some(update_robot(robot, 'L', num_prev, robot.dir.get_prev()));
    }

    None
}

pub fn run_puzzle() {
    let mut file = File::open("input_day17.txt").expect("Failed to open input_day17.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let vec: Vec<i64> = ops_string.split(',').map(|text| text.trim().parse().unwrap()).collect();

    let mut program = Program::new(&vec, VecDeque::new());
    let mut program_clone = program.clone();

    let map_output = program.run_program(Vec::new());

    let map_string: String = map_output.iter().map(|v| (*v as u8) as char).collect();
    println!("{}", map_string);

    let mut map_vec: Vec<Vec<TileType>> = Vec::new();

    let mut robot = Robot::new(0, 0, Direction::North);

    let mut y = 0;
    for l in map_string.trim().lines() {
        let mut new_line = Vec::new();
        let mut x = 0;
        for c in l.chars() {
            let tile = match c {
                '.' => TileType::Empty,
                '#' => TileType::Scaffold,
                '^' => {
                    robot = Robot::new(x, y, Direction::North);
                    TileType::Scaffold
                }
                '<' => {
                    robot = Robot::new(x, y, Direction::West);
                    TileType::Scaffold
                }
                '>' => {
                    robot = Robot::new(x, y, Direction::East);
                    TileType::Scaffold
                }
                'v' => {
                    robot = Robot::new(x, y, Direction::South);
                    TileType::Scaffold
                }
                o => panic!("Unhandled Tile: {}", o),
            };
            new_line.push(tile);
            x += 1;
        }
        map_vec.push(new_line);
        y += 1;
    }

    let map = Map::new(map_vec);

    let mut all_insts = Vec::new();
    while let Some(inst) = get_next_instruction(&map, &mut robot) {
        all_insts.push(inst);
    }

    let mut inst_str = String::new();
    for i in &all_insts {
        inst_str += &i.to_string();
    }

    println!("Instructions: {}", inst_str);

    // Search for 3 non overlapping patterns <= 20 bytes that make up the instruction list

    // Shamefully solved by hand in a text editor :`D
    let result_str = "A,A,B,C,B,C,B,C,B,A\nL,10,L,8,R,8,L,8,R,6\nR,6,R,8,R,8\nR,6,R,6,L,8,L,10\nn\n";

    let input: Vec<i64> = result_str.chars().map(|c| c as i64).collect();

    program_clone.state[0] = 2;
    let output = program_clone.run_program(input);

    let map_string: String = output.iter().map(|v| (*v as u8) as char).collect();
    println!("{}", map_string);

    for v in output {
        if *v > 255 {
            println!("Output: {}", v);
        }
    }
}
