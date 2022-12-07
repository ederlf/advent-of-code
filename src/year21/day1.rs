use super::super::utils;

fn increases(values: Vec<i32>) -> String {
    let negative_differences: Vec<i32> = values
        .windows(2)
        .map(|x| x[0] - x[1])
        .filter(|x| *x < 0)
        .collect();
    negative_differences.len().to_string()
}

fn part1(input: String) -> String {
    let values: Vec<i32> = utils::parse_number_lines(input);
    increases(values)
}

fn part2(input: String) -> String {
    let values: Vec<i32> = utils::parse_number_lines(input);
    let window_sum: Vec<i32> = values.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    increases(window_sum)
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
