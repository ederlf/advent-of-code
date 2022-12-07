fn part1(input: String) -> String {
    let positions: Vec<usize> = input
        .trim_end_matches(&['\r', '\n'][..])
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut min_fuel = usize::MAX;
    for pos in positions.iter() {
        let fuel = positions.iter().fold(0, |acc, x| acc + x.abs_diff(*pos));
        min_fuel = if fuel < min_fuel { fuel } else { min_fuel };
    }
    min_fuel.to_string()
}

fn natural_sum(v: usize) -> usize {
    (v * (v + 1)) / 2
}

fn part2(input: String) -> String {
    let positions: Vec<usize> = input
        .trim_end_matches(&['\r', '\n'][..])
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut min_fuel = usize::MAX;
    for pos in 0..*positions.iter().max().unwrap() {
        let fuel = positions
            .iter()
            .fold(0, |acc, x| acc + natural_sum(x.abs_diff(pos)));
        min_fuel = if fuel < min_fuel { fuel } else { min_fuel };
    }
    min_fuel.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
