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

    let (x, y) = (22, 25);
    let mut dest_count = 0;

    while aster_list.len() != 1 {
        let mut visible_list: HashSet<(i64, i64)> = HashSet::new();

        for (ox, oy) in aster_list.iter().copied() {
            if x == ox && y == oy {
                continue;
            }

            let (dist_x, dist_y) = (ox - x, oy - y);
            let gcd = gcd(dist_x, dist_y);
            let (step_x, step_y) = (dist_x / gcd, dist_y / gcd);

            let (mut cur_x, mut cur_y) = (ox - step_x, oy - step_y);
            let mut visible = true;
            while cur_x != x || cur_y != y {
                if aster_list.contains(&(cur_x, cur_y)) {
                    visible = false;
                    break;
                }
                cur_x -= step_x;
                cur_y -= step_y;
            }

            if visible {
                visible_list.insert((ox, oy));
            }
        }

        let mut sorted_visible = Vec::new();
        for (ox, oy) in visible_list.iter().copied() {
            let t = (oy-y) as f64;
            let mut angle = t.atan2((ox-x) as f64).to_degrees() + 90f64;
            angle = if angle < 0f64 { 360f64 + angle } else if angle > 360f64 { angle - 360f64 } else { angle };
            sorted_visible.push((ox, oy, angle));
        }

        sorted_visible.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        for v in &sorted_visible {
            aster_list.remove(&(v.0, v.1));
            dest_count += 1;
            // println!("Destroyed asteroid({}) at ({}, {}), degrees:{}", dest_count, v.0, v.1, v.2);
            if dest_count == 200 {
                println!("Destroyed 200th asteroid at ({}, {})", v.0, v.1);
                return;
            }
        }
    }
}
