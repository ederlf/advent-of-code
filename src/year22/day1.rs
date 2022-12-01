fn parse_elves(input: String) -> Vec<usize> {
    input.lines().fold(Vec::new(), |mut acc, x| {
            if x == "" || acc.is_empty() {
                acc.push(Vec::new());
                return acc
            }
            acc.last_mut().unwrap().push(x.parse::<usize>().unwrap());
            acc
        }).iter()
        .map(|x| x.iter().sum::<usize>()).collect()
}

fn part1(input: String)  -> String {
    let elves = parse_elves(input);
    let max: usize  = *elves.iter().max().unwrap();
    max.to_string()
}

fn part2(input: String) -> String {
    let mut elves = parse_elves(input);
    elves.sort_by(|a, b| b.cmp(a));
    let total = elves.iter().take(3).sum::<usize>();
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
