use std::str;

fn operate_crane(input: String, part1: bool) -> String {
    let stack_lines = input.lines().filter(|x| x.contains("["));
    let instructions = input
        .lines()
        .filter(|x| x.contains("move"))
        .collect::<Vec<&str>>();

    let mut stacks: Vec<Vec<&str>> = stack_lines.rev().fold(Vec::new(), |mut acc, s| {
        let elements: Vec<&str> = s
            .as_bytes()
            .chunks(4)
            .map(|x| str::from_utf8(x).unwrap())
            .collect();
        if acc.is_empty() {
            for _ in 0..elements.len() {
                acc.push(Vec::new());
            }
        }
        for (i, e) in elements.iter().enumerate() {
            let clean = e.trim();
            if !clean.is_empty() {
                acc[i].push(clean.trim_matches(|x| x == '[' || x == ']'));
            }
        }
        acc
    });

    for inst in instructions {
        let mv: Vec<usize> = inst
            .split_whitespace()
            .filter(|x| !(x.contains("move") || x.contains("from") || x.contains("to")))
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut st = Vec::new();
        for _ in 0..mv[0] {
            let v = stacks[mv[1] - 1].pop().unwrap();
            if part1 {
                stacks[mv[2] - 1].push(v);
            } else {
                st.push(v);
            }
        }

        // st will have contents only if part 2
        for e in st.iter().rev() {
            stacks[mv[2] - 1].push(e);
        }
    }

    let res = stacks
        .iter()
        .map(|x| x.last().unwrap())
        .fold(String::new(), |acc, x| acc + x);
    res
}

fn part1(input: String) -> String {
    operate_crane(input, true)
}

fn part2(input: String) -> String {
    operate_crane(input, false)
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
