use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Element {
    name: String,
    quantity: i64,
}

impl Element {
    fn new(name: String, quantity: i64) -> Element {
        Element { name, quantity }
    }

    fn from_string(string: &str) -> Element {
        let quantity_and_elem: Vec<&str> = string.split(' ').collect();
        if quantity_and_elem.len() != 2 {
            panic!("String doesn't contain quantity and element");
        }

        Element::new(quantity_and_elem[1].to_string(), quantity_and_elem[0].parse().unwrap())
    }
}

pub fn run_puzzle() {
    let mut file = File::open("input_day14.txt").expect("Failed to open input_day14.txt");
    let mut reacts_string = String::new();
    file.read_to_string(&mut reacts_string).unwrap();

    let mut reactions: Vec<(Element, Vec<Element>)> = Vec::new();

    for line in reacts_string.lines() {
        let elems_and_res: Vec<String> = line.split("=>").map(|l| l.trim().to_string()).collect();
        if elems_and_res.len() != 2 {
            panic!("len() != 2");
        }

        let elements: Vec<Element> = elems_and_res[0].split(',').map(|e| Element::from_string(e.trim())).collect();
        let result = Element::from_string(&elems_and_res[1]);

        reactions.push((result, elements));
    }

    let mut elements_needed: HashMap<String, i64> = HashMap::new();
    elements_needed.insert("FUEL".to_string(), 1);

    let mut finished = false;
    while !finished {
        let mut new_elements = elements_needed.clone();
        for (e, quantity) in &elements_needed {
            if *e == "ORE" || *quantity <= 0 {
                continue;
            }

            for r in &reactions {
                if r.0.name == *e {
                    let num_reactions;

                    if *quantity % r.0.quantity != 0 {
                        num_reactions = (*quantity / r.0.quantity) + 1;
                    } else {
                        num_reactions = *quantity / r.0.quantity;
                    }

                    for n in &r.1 {
                        let elem_entry = new_elements.entry(n.name.to_string()).or_insert(0);
                        *elem_entry += num_reactions * n.quantity;
                    }

                    let res_entry = new_elements.entry(e.to_string()).or_insert(0);
                    *res_entry -= num_reactions * r.0.quantity;
                }
            }
        }

        elements_needed = new_elements;

        finished = true;
        for (e, quantity) in &elements_needed {
            if e == "ORE" {
                continue;
            }
            if *quantity > 0 {
                finished = false;
                break;
            }
        }
    }

    println!("Needs {} ORE", elements_needed[&"ORE".to_string()]);
}
