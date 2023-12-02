use std::collections::HashSet;

fn line_number_only_digits(line: String) -> i32 {
    let mut numbers = Vec::<String>::new();
    for c in line.chars() {
        if c.is_digit(10) {
            numbers.push(c.to_string());
        }
    }

    let first = numbers.first().unwrap();
    let second = match numbers.last() {
        Some(x) => x,
        None => "",
    };
    if second.is_empty() {
        let res = (first.clone() + &first).parse::<i32>().unwrap();
        return res;
    }

    let res = (first.clone() + &second).parse::<i32>().unwrap();
    res
}

fn convert(number: &str) -> String {
    let v = match number {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => number,
    };
    v.to_string()
}

fn line_number_digits_words(line: String) -> i32 {
    let words = HashSet::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);

    let mut numbers = Vec::new();
    for (i, c) in line.chars().enumerate() {
        let mut number = Vec::new();
        number.push(c);
        if c.is_digit(10) {
            numbers.push(c.to_string());
            number.clear();
            continue;
        }
        for next in line.chars().skip(i + 1) {
            number.push(next);
            let num: String = number.iter().collect();
            if words.contains(num.as_str()) {
                let v = convert(num.as_str());
                numbers.push(v);
                number.clear();
                continue;
            }
        }
    }

    let first = numbers.first().unwrap();
    let second = match numbers.last() {
        Some(x) => x,
        None => "",
    };
    if second.is_empty() {
        let res = (first.clone() + &first).parse::<i32>().unwrap();
        return res;
    }

    let res = (first.clone() + &second).parse::<i32>().unwrap();
    res
}

fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| line_number_only_digits(line.to_string()))
        .sum::<i32>()
        .to_string()
}

fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| line_number_digits_words(line.to_string()))
        .sum::<i32>()
        .to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
