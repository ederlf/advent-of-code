use std::collections::HashMap;
use std::collections::HashSet;

fn points() -> HashMap<u8, usize> {
    let mut scores = HashMap::new();
    for n in "A".as_bytes()[0].."[".as_bytes()[0] {
        let diff = (n - "A".as_bytes()[0]) as usize;
        scores.insert(n, 27 + diff);
    }
    for n in "a".as_bytes()[0].."{".as_bytes()[0] {
        let diff = (n - "a".as_bytes()[0]) as usize;
        scores.insert(n, 1 + diff);
    }
    scores
}

fn part1(input: String) -> String {
    let values = points();
    let backpacks = input.lines().fold(Vec::new(), |mut acc, backpack| {
        let compartments: Vec<HashSet<&u8>> = backpack
            .as_bytes()
            .chunks(backpack.len() / 2)
            .map(|x| -> HashSet<&u8> { HashSet::from_iter(x.iter()) })
            .collect();
        acc.push(compartments);
        acc
    });
    let repeated: Vec<usize> = backpacks
        .iter()
        .map(|x| x[0].intersection(&x[1]).fold(0, |acc, v| acc + values[v]))
        .collect();
    let total: usize = repeated.iter().sum();
    total.to_string()
}

fn find_intersections(backpack: Vec<HashSet<&u8>>, values: &HashMap<u8, usize>) -> usize {
    let result: HashSet<&u8> = backpack[0].intersection(&backpack[1]).copied().collect();
    result
        .intersection(&backpack[2])
        .fold(0, |acc, v| acc + values[v])
}

fn part2(input: String) -> String {
    let values = points();
    let backpacks: Vec<HashSet<&u8>> = input
        .lines()
        .map(|x| -> HashSet<&u8> { HashSet::from_iter(x.as_bytes().iter()) })
        .collect();
    let total = backpacks
        .chunks(3)
        .map(|x| find_intersections(x.to_vec(), &values))
        .sum::<usize>();
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
