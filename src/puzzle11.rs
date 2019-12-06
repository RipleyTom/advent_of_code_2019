use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn get_orbiting<'a>(obj: &str, orbits: &'a Vec<(String, String)>) -> &'a String {
    &orbits.iter().find(|&orbit| orbit.1 == *obj).unwrap().0
}

fn calculate_orbits(obj: &String, orbits: &Vec<(String, String)>, num_orbits: &mut HashMap<String, u64>) -> u64 {
    if obj == "COM" {
        return 0;
    }

    if let Some(num) = num_orbits.get(obj) {
        return *num;
    }

    // Find object it's orbiting
    let orbiting = get_orbiting(obj, orbits);

    // Current object is one orbit away from the object it's orbiting
    let obj_orbits = calculate_orbits(orbiting, orbits, num_orbits) + 1;
    num_orbits.insert(obj.clone(), obj_orbits);

    obj_orbits
}

pub fn run_puzzle() {
    let mut file = File::open("input_5.txt").expect("Failed to open input_5.txt");
    let mut ops_string = String::new();
    file.read_to_string(&mut ops_string).unwrap();

    let mut orbits: Vec<(String, String)> = Vec::new();

    let lines = ops_string.lines();
    for line in lines {
        let elements: Vec<&str> = line.split(')').collect();
        orbits.push((elements[0].to_string(), elements[1].to_string()));
    }

    let mut num_orbits: HashMap<String, u64> = HashMap::new();

    for orbit in &orbits {
        calculate_orbits(&orbit.1, &orbits, &mut num_orbits);
    }

    let total: u64 = num_orbits.iter().map(|orbit| orbit.1).sum();

    println!("Total orbits is: {}", total);
}
