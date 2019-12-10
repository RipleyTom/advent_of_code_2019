use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn gcd(a: i64, b: i64) -> i64 {
    let (mut tmp_a, mut tmp_b) = (a, b);
    while tmp_b != 0 {
        let t = tmp_b;
        tmp_b = tmp_a % tmp_b;
        tmp_a = t;
    }

    if tmp_a < 0 {
        -tmp_a
    } else {
        tmp_a
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_day10.txt").expect("Failed to open input_day10.txt");
    let mut map_string = String::new();
    file.read_to_string(&mut map_string).unwrap();

    let map: Vec<Vec<u8>> = map_string.lines().map(|line| line.trim().as_bytes().to_vec()).collect();

    let width = map[0].len();
    let height = map.len();

    let mut aster_list: HashSet<(i64, i64)> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == '#' as u8 {
                aster_list.insert((x as i64, y as i64));
            }
        }
    }

    let mut max_visible = 0;
    let (mut max_x, mut max_y) = (0, 0);

    for (x, y) in aster_list.iter().copied() {
        let mut visible = 0;
        for (ox, oy) in aster_list.iter().copied() {
            if x == ox && y == oy {
                continue;
            }

            let (dist_x, dist_y) = (ox - x, oy - y);
            let gcd = gcd(dist_x, dist_y);
            let (step_x, step_y) = (dist_x / gcd, dist_y / gcd);

            let (mut cur_x, mut cur_y) = (ox - step_x, oy - step_y);
            let mut found_obstacle = false;
            // println!("Looping from:({}, {}) to ({}, {}) with steps({}, {})", cur_x, cur_y, x, y, step_x, step_y);
            while cur_x != x || cur_y != y {
                if aster_list.contains(&(cur_x, cur_y)) {
                    found_obstacle = true;
                    break;
                }
                cur_x -= step_x;
                cur_y -= step_y;
            }

            if !found_obstacle {
                visible += 1;
            }
        }

        if visible > max_visible {
            max_visible = visible;
            max_x = x;
            max_y = y;
        }
    }

    println!("Max asteroids in view is: ({},{}) with {}", max_x, max_y, max_visible);
}
