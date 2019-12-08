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

    let mut num_chars = Vec::new();

    for layer in &layers {
        let mut chars = vec![0; 10];
        for c in layer.chars() {
            chars[c.to_digit(10).unwrap() as usize] += 1;
        }
        num_chars.push(chars);
    }

    let (mut num_zero, mut layer) = (std::u64::MAX, 0);

    for i in 0..num_chars.len() {
        if num_chars[i][0] < num_zero {
            num_zero = num_chars[i][0];
            layer = i;
        }
    }

    let result = num_chars[layer][1] * num_chars[layer][2];

    println!("Result is: {}", result);
}
