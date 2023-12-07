use itertools::join;

fn calculate_wins(time: usize, distance: usize) -> usize {
    let mut wait = time - 1;
    let mut wins = 0;
    while wait != 0 {
        let remaining = time - wait;
        let dist = wait * remaining;
        if dist > distance {
            wins += 1;
        }
        wait -= 1;
    }
    wins
}

fn part1(input: String) -> String {
    let mut races = input.split('\n');
    let races_times: Vec<usize> = races
        .next()
        .expect("No more lines")
        .split(':')
        .nth(1)
        .expect("No times")
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Not a number"))
        .collect();
    let mut distances: Vec<usize> = races
        .next()
        .expect("No more lines")
        .split(':')
        .nth(1)
        .expect("No times")
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Not a number"))
        .collect();
    let mut wins: Vec<usize> = Vec::new();
    for (time, distance) in races_times.iter().zip(distances.iter_mut()) {
        wins.push(calculate_wins(*time, *distance));
    }
    let total: usize = wins.iter().fold(1, |acc, e| acc * e);
    total.to_string()
}

fn part2(input: String) -> String {
    let mut races = input.split('\n');
    let races_times = races
        .next()
        .expect("No more lines")
        .split(':')
        .nth(1)
        .expect("No times")
        .trim()
        .split_whitespace();
    let distances = races
        .next()
        .expect("No more lines")
        .split(':')
        .nth(1)
        .expect("No times")
        .trim()
        .split_whitespace();
    let time: usize = join(races_times, "").parse::<usize>().unwrap();
    let distance: usize = join(distances, "").parse::<usize>().unwrap();
    calculate_wins(time, distance).to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
