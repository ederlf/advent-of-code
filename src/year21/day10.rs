use std::collections::HashMap;

fn define_matchings() -> HashMap<char, char> {
    let mut matching = HashMap::new();
    matching.insert('{', '}');
    matching.insert('[', ']');
    matching.insert('(', ')');
    matching.insert('<', '>');
    matching
}

fn corrupted_points(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn closing_points(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
// .0 corrupted, .1 completion
fn points(input: &String) -> (usize, usize) {
    let matchings = define_matchings();
    let lines =  input.split("\n").filter(|x| !x.is_empty());
    let mut corrupted = 0;
    let mut completion_scores = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            if c == '{' || c == '[' || c == '(' || c == '<' {
                stack.push(c);
            } else {
                let top = stack.last().unwrap();
                if c != *matchings.get(top).unwrap() {
                    corrupted += corrupted_points(c);
                    corrupt = true;
                    break;
                }
                else {
                    stack.pop();
                }
            }
        }
        if !corrupt {
            let mut completed_line_score = 0;
            for c in stack.iter().rev() {
                completed_line_score *= 5;
                completed_line_score += closing_points(*c);
            }
            completion_scores.push(completed_line_score);
        }
    }
    let idx = completion_scores.len() / 2;
    completion_scores.sort();
    (corrupted, completion_scores[idx])
}

fn part1(input: String)  -> String {
    points(&input).0.to_string()
}

fn part2(input: String) -> String {
    points(&input).1.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
