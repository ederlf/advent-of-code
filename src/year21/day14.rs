use std::collections::HashMap;

fn apply_rules<'a>(sequence: Vec<&'a str>, rules: &'a HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut new_seq = Vec::new();
    for pair in sequence.windows(2) {
        let duo = pair.join("");
        new_seq.push(pair[0]);
        if rules.contains_key(&duo.as_str()) {
            new_seq.push(rules[duo.as_str()]);
        }
    }
    new_seq.push(sequence.last().unwrap());
    new_seq
}

fn part1(input: String)  -> String {
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut seq: Vec<&str> = lines[0].split("").filter(|x| !x.is_empty()).collect();
    let rules = &lines[1..];
    let mut rule_map = HashMap::new();
    for rule in rules.iter() {
        let formula: Vec<&str> = rule.split(" ").collect();
        rule_map.insert(formula[0], formula[2]);
    }

    for _ in 0..10 {
        seq = apply_rules(seq, &rule_map);
    }

    let mut count: HashMap<&str, usize> = HashMap::new();
    for c in seq {
        *count.entry(c).or_default() += 1;
    }


    let max = count.iter().max_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();
    let min = count.iter().min_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();
    (max-min).to_string()
}

fn run_step(pairs: &HashMap<String, usize>, rules: &HashMap<&str, &str>) -> HashMap<String, usize> {
    let mut updated_pairs: HashMap<String, usize> = HashMap::new();
    for (pair, count) in pairs {
        if rules.contains_key(pair.as_str()) {
            let p1 = pair.chars().nth(0).unwrap().to_string() + &rules[pair.as_str()].to_string();
            updated_pairs.entry(p1).and_modify(|cnt| *cnt += count).or_insert(*count);
            let p2 = rules[pair.as_str()].to_string() + &pair.chars().nth(1).unwrap().to_string();
            updated_pairs.entry(p2).and_modify(|cnt| *cnt += count).or_insert(*count);
        }

    }
    updated_pairs
}

fn part2(input: String) -> String {
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let seq: Vec<&str> = lines[0].split("").filter(|x| !x.is_empty()).collect();
    let rules = &lines[1..];
    let mut rule_map = HashMap::new();
    for rule in rules.iter() {
        let formula: Vec<&str> = rule.split(" ").collect();
        rule_map.insert(formula[0], formula[2]);
    }
    let mut pairs: HashMap<String, usize> = HashMap::new();
    for pair in seq.windows(2) {
        pairs.entry(pair.join("")).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    for _ in 0..40 {
        pairs = run_step(&pairs, &rule_map);
    }

    let mut counts = HashMap::new();
    for (pair, count) in pairs {
        let v = pair.chars().nth(0).unwrap();
        counts.entry(v).and_modify(|cnt| *cnt += count).or_insert(count);
    }

    let last = seq.last().unwrap();
    *counts.entry(last.chars().nth(0).unwrap()).or_default() += 1;

    let max = counts.iter().max_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();
    let min = counts.iter().min_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();

    (max - min).to_string()
}


pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
