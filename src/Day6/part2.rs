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

fn list_systems(obj: &str, orbits: &Vec<(String, String)>) -> Vec<String> {
    let mut vec = Vec::new();

    if obj == "COM" {
        vec.push("COM".to_string());
        return vec;
    }

    for orbit in orbits {
        if orbit.1 == obj {
            vec.push(obj.to_string());
            vec.extend(list_systems(&orbit.0, orbits));
            return vec;
        }
    }

    panic!("Object {} not found orbiting anything", obj);
}

pub fn run_puzzle() {
    let mut file = File::open("input_day6.txt").expect("Failed to open input_day6.txt");
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

    let you_orbiting = get_orbiting("YOU", &orbits);
    let san_orbiting = get_orbiting("SAN", &orbits);

    let you_systems = list_systems(&you_orbiting, &orbits);
    let san_systems = list_systems(&san_orbiting, &orbits);

    for sys in &you_systems {
        if let Some(common) = san_systems.iter().find(|&dasys| *dasys == *sys) {
            let distance_you = *num_orbits.get(you_orbiting).unwrap();
            let distance_san = *num_orbits.get(san_orbiting).unwrap();
            let distance_common = *num_orbits.get(common).unwrap();    

            let distance = (distance_you - distance_common) + (distance_san - distance_common);
            println!("Distance between systems: {}", distance);
            return;
        }
    }

    println!("No common system found?!?");
}
