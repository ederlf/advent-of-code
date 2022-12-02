use std::collections::HashMap;

fn base_points(c: &str) -> usize {
    let point = match c {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };
    point
}

fn points() -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for (i, v) in ["Z", "X", "Y"].iter().enumerate() {
        m.insert(format!("{} {}", "A", *v), base_points(v) + i * 3);
    }

    for (i, v) in ["X", "Y", "Z"].iter().enumerate() {
        m.insert(format!("{} {}", "B", *v), base_points(v) + i * 3);
    }

    for (i, v) in ["Y", "Z", "X"].iter().enumerate() {
        m.insert(format!("{} {}", "C", *v), base_points(v) + i * 3);
    }
    m
}

fn combinations(opponent: &str, decision: &str) -> usize {
    let res = match decision {
        "X" => match opponent {
            "A" => base_points("C"),
            "B" => base_points("A"),
            "C" => base_points("B"),
            _ => 0,
        },
        "Y" => match opponent {
            "A" => base_points("A") + 3,
            "B" => base_points("B") + 3,
            "C" => base_points("C") + 3,
            _ => 0,
        },
        "Z" => match opponent {
            "A" => base_points("B") + 6,
            "B" => base_points("C") + 6,
            "C" => base_points("A") + 6,
            _ => 0,
        },
        _ => 0,
    };
    println!("{} {} {}", opponent, decision, res);
    res
}

fn part1(input: String) -> String {
    let scores = points();
    let total = input
        .lines()
        .fold(0, |acc, x| acc + scores.get(&x.to_string()).unwrap());
    total.to_string()
}

fn part2(input: String) -> String {
    let total = input
        .lines()
        .map(|x| x.split(" ").collect())
        .fold(0, |acc, x: Vec<&str>| acc + combinations(x[0], x[1]));
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
