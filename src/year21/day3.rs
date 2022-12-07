fn count_ones(lines: &Vec<&str>) -> Vec<u32> {
    let mut count_ones: Vec<u32> = Vec::new();
    for _ in 0..lines[0].len() {
        count_ones.push(0);
    }

    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            count_ones[i] += c.to_digit(10).unwrap();
        }
    }
    count_ones
}

fn find_common<'a>(
    lines: &Vec<&'a str>,
    nth_bit: usize,
    ones: bool,
) -> (Vec<&'a str>, Vec<&'a str>) {
    let num_lines = lines.len() as u32;
    let count_ones = count_ones(lines);

    let build_str = |x: &u32| match ones {
        true => {
            if num_lines - *x > *x {
                "0"
            } else {
                "1"
            }
        }
        false => {
            if num_lines - *x > *x {
                "1"
            } else {
                "0"
            }
        }
    };

    let most_common: Vec<&str> = count_ones.iter().map(build_str).collect();

    let lines_left = lines
        .clone()
        .into_iter()
        .filter(|x| {
            x.chars()
                .nth(nth_bit)
                .unwrap()
                .to_string()
                .eq(most_common[nth_bit])
        })
        .collect();

    (most_common, lines_left)
}

fn part1(input: String) -> String {
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let (most_common, _) = find_common(&lines, 0, true);
    let (least_common, _) = find_common(&lines, 0, false);

    let most_common_value = isize::from_str_radix(most_common.join("").as_str(), 2).unwrap();
    let least_common_value = isize::from_str_radix(least_common.join("").as_str(), 2).unwrap();
    let result = most_common_value * least_common_value;
    result.to_string()
}

fn consume(mut lines: Vec<&str>, most: bool) -> isize {
    let mut i = 0;
    while lines.len() > 1 {
        lines = find_common(&lines, i, most).1;
        i += 1;
    }
    isize::from_str_radix(lines.join("").as_str(), 2).unwrap()
}

fn part2(input: String) -> String {
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let oxygen = consume(lines.clone(), true);
    let co2 = consume(lines.clone(), false);
    let result = oxygen * co2;
    result.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
