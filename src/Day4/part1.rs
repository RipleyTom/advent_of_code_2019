pub fn test_number(num: u64) -> bool {
    let mut last_digit = num % 10;
    let mut number = num / 10;
    let mut pair_found = false;

    while number != 0 {
        let current_digit = number % 10;
        if current_digit > last_digit {
            return false;
        }

        if last_digit == current_digit {
            pair_found = true;
        }

        last_digit = current_digit;
        number /= 10;
    }

    if pair_found {
        return true;
    }

    false
}

pub fn run_puzzle() {
    let min = 264360;
    let max = 746325;

    let mut count = 0;

    for num in min..max + 1 {
        if test_number(num) {
            count += 1;
        }
    }

    println!("Result: {}", count);
}
