pub fn test_number(num: u64) -> bool {
    let mut last_digit = num % 10;
    let mut number = num / 10;
    let (mut min_pair, mut cur_pair) = (9, 1);

    while number != 0 {
        let current_digit = number % 10;
        if current_digit > last_digit {
            return false;
        }

        if last_digit == current_digit {
            cur_pair += 1;
        } else {
            if cur_pair > 1 && cur_pair < min_pair {
                min_pair = cur_pair;
            }
            cur_pair = 1;
        }

        last_digit = current_digit;
        number /= 10;
    }

    if cur_pair > 1 && cur_pair < min_pair {
        min_pair = cur_pair;
    }

    if min_pair == 2 {
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

    println!("Puzzle 8 result: {}", count);
}
