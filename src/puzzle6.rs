use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_puzzle() {
    let file = File::open("input_3.txt").expect("Failed to open input_3.txt");
    let br = BufReader::new(file);

    let mut wires: HashMap<(i64, i64), (u64, u64)> = HashMap::new();
    let mut curwire = 1;

    for line in br.lines() {
        let mut curpos = (0, 0);
        let mut steps = 0;

        for op in line.unwrap().split(',') {
            let diff = i64::from_str_radix(&op[1..], 10).unwrap();

            match op.get(0..1).unwrap() {
                "L" => {
                    for x in (((curpos.0 + 1) - diff)..(curpos.0 + 1)).rev() {
                        let wire = wires.entry((x, curpos.1)).or_insert((0, 0));
                        if (wire.0 & curwire) == 0 {
                            (*wire).0 |= curwire;
                            (*wire).1 += steps;
                        }
                        steps += 1;
                    }
                    curpos.0 -= diff;
                }
                "R" => {
                    for x in curpos.0..curpos.0 + diff {
                        let wire = wires.entry((x, curpos.1)).or_insert((0, 0));
                        if (wire.0 & curwire) == 0 {
                            (*wire).0 |= curwire;
                            (*wire).1 += steps;
                        }
                        steps += 1;
                    }
                    curpos.0 += diff;
                }
                "U" => {
                    for y in (((curpos.1 + 1) - diff)..(curpos.1 + 1)).rev() {
                        let wire = wires.entry((curpos.0, y)).or_insert((0, 0));
                        if (wire.0 & curwire) == 0 {
                            (*wire).0 |= curwire;
                            (*wire).1 += steps;
                        }
                        steps += 1;
                    }
                    curpos.1 -= diff;
                }
                "D" => {
                    for y in curpos.1..curpos.1 + diff {
                        let wire = wires.entry((curpos.0, y)).or_insert((0, 0));
                        if (wire.0 & curwire) == 0 {
                            (*wire).0 |= curwire;
                            (*wire).1 += steps;
                        }
                        steps += 1;
                    }
                    curpos.1 += diff;
                }
                _ => panic!("Unsupported pattern!"),
            }
        }
        curwire <<= 1;
    }

    let mut curdist: u64 = std::u64::MAX;

    for point in wires {
        if (point.1).0 == 3 {
            if (point.1).1 != 0 && (point.1).1 < curdist {
                curdist = (point.1).1;
            }
        }
    }

    println!("Puzzle 6 result: {}", curdist);
}
