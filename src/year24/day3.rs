use regex::Regex;

fn part1(input: String)  -> String {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();


    let mut total_sum: u64 = 0;

    for cap in re.captures_iter(&input) {
        let num1: u64 = cap[1].parse().unwrap();
        let num2: u64 = cap[2].parse().unwrap();
        let product = num1 * num2;
        total_sum += product;
    }

    total_sum.to_string()
}

fn part2(input: String) -> String {
    let pattern = r"(don't\(\)|do\(\))";
    let re = Regex::new(pattern).unwrap();
    let mul_pattern = r"mul\((\d+),(\d+)\)";
    let mul_re = Regex::new(mul_pattern).unwrap();

    let mut last_index = 0;
    let mut total_sum: u64 = 0;

    let mut mul = true;
    for mat in re.find_iter(&input) {
        // Get the text before the current delimiter
        let before_delimiter = &input[last_index..mat.start()];
        if mul {
            for cap in mul_re.captures_iter(&before_delimiter) {
                let num1: u64 = cap[1].parse().unwrap();
                let num2: u64 = cap[2].parse().unwrap();
                let product = num1 * num2;
                total_sum += product;
            }
        }
        if mat.as_str().contains("don't") {
            mul = false;
        } else {
            mul = true;
        }

        last_index = mat.end();
    }

    if last_index < input.len() {
        if mul {
            for cap in mul_re.captures_iter(&input[last_index..]) {
                let num1: u64 = cap[1].parse().unwrap();
                let num2: u64 = cap[2].parse().unwrap();
                let product = num1 * num2;
                total_sum += product;
            }
        }
    }

    total_sum.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
