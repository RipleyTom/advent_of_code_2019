use std::fs::File;
use std::io::Read;

pub fn run_puzzle() {
    let mut file = File::open("input_day8.txt").expect("Failed to open input_day8.txt");
    let mut img_str = String::new();
    file.read_to_string(&mut img_str).unwrap();

    let wide = 25;
    let tall = 6;

    let mut layers = Vec::new();

    let mut cur_str = img_str.trim();

    while cur_str.len() > wide * tall {
        let (a, b) = cur_str.split_at(wide * tall);
        layers.push(a);
        cur_str = b;
    }
    layers.push(cur_str);

    let mut img_result = vec!['2'; wide * tall];

    for i in 0..wide * tall {
        for layer in &layers {
            img_result[i] = layer.chars().nth(i).unwrap();
            if img_result[i] != '2' {
                break;
            }
        }
    }

    for y in 0..tall {
        let mut rstr = String::new();
        for x in 0..wide {
            match img_result[(y * wide) + x] {
                '0' | '2' => rstr.push(' '),
                '1' => rstr.push('X'),
                _ => panic!("Unexpected char"),
            }
        }
        println!("{}", rstr);
    }
}
