static PATTERN: [i64; 4] = [0, 1, 0, -1];

fn process_signal(cur_phase: &Vec<i64>) -> Vec<i64> {
    let mut new_phase: Vec<i64> = vec![0; cur_phase.len()];

    for i in 0..cur_phase.len() {
        let mut result = 0;
        for j in 0..cur_phase.len() {
            let cur_pattern = ((j + 1) / (i + 1)) % 4;
            result += cur_phase[j] * PATTERN[cur_pattern];
        }
        new_phase[i] = (result % 10).abs();
    }

    new_phase
}

fn print_phase(phase: &Vec<i64>) {
    let mut res_str = String::new();

    for i in phase {
        res_str += &i.to_string();
    }

    println!("Current phase: {}", res_str);
}

pub fn run_puzzle() {
    let input = "59776034095811644545367793179989602140948714406234694972894485066523525742503986771912019032922788494900655855458086979764617375580802558963587025784918882219610831940992399201782385674223284411499237619800193879768668210162176394607502218602633153772062973149533650562554942574593878073238232563649673858167635378695190356159796342204759393156294658366279922734213385144895116649768185966866202413314939692174223210484933678866478944104978890019728562001417746656699281992028356004888860103805472866615243544781377748654471750560830099048747570925902575765054898899512303917159138097375338444610809891667094051108359134017128028174230720398965960712";

    let mut phase: Vec<i64> = Vec::new();
    for i in input.chars() {
        phase.push(i.to_digit(10).unwrap() as i64);
    }

    for _ in 0..100 {
        phase = process_signal(&phase);
    }
    print_phase(&phase);
}
