use std::collections::HashMap;

fn get_parent(dir: &String) -> String {
    if dir == "/" {
        return "".to_string();
    }

    let mut prev = dir.split("/").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    prev.pop();
    if prev.is_empty() {
        return "/".to_string()
    }

    return format!("/{}/", prev.join("/"));
}

fn cd(cur: &String, dir: &String) -> String {
    if dir == ".." {
        return get_parent(cur)
    }

    if dir == "/" {
        return dir.to_owned()
    }

    return format!("{}{}/", cur, dir);
}

fn process_line<'a>(l: &'a str, cur_dir: String, sizes: &mut HashMap<String, usize>) -> String{
    let parts: Vec<&str> = l.split(" ").collect();
    if parts[0] == "cd" {
        let dir: Vec<&str> = l.split(" ").collect();
        return cd(&cur_dir, &dir[1].to_string())
    }

    if parts[0] != "ls" && parts[0] != "dir" {
        if !sizes.contains_key(&cur_dir) {
            sizes.insert(cur_dir.to_string(), 0);
        }
        sizes.insert(cur_dir.to_string(), sizes.get(&cur_dir).unwrap() + parts[0].parse::<usize>().unwrap());

        let mut parent = get_parent(&cur_dir);
        while parent != "" {
            if !sizes.contains_key(&parent) {
                sizes.insert(parent.to_string(), 0);
            }
            sizes.insert(parent.to_string(), sizes.get(&parent).unwrap() + parts[0].parse::<usize>().unwrap());
            parent = get_parent(&parent);
        }
    }
    cur_dir
}

fn part1(input: String) -> String {
    let mut sizes = HashMap::new();
    let clean_lines: Vec<&str> = input
        .lines()
        .map(|x| x.trim_matches(|x| x == '$' || x == ' '))
        .collect();
    let mut cur_dir = "".to_string();
    for l in clean_lines {
        cur_dir = process_line(l, cur_dir, &mut sizes);
    }

    let total = sizes.values().filter(|x| **x <= 100000 as usize).sum::<usize>();
    total.to_string()
}

fn part2(input: String) -> String {
    let mut sizes = HashMap::new();
    let clean_lines: Vec<&str> = input
        .lines()
        .map(|x| x.trim_matches(|x| x == '$' || x == ' '))
        .collect();
    let mut cur_dir = "".to_string();
    for l in clean_lines {
        cur_dir = process_line(l, cur_dir, &mut sizes);
    }

    let required_space = 30000000 - (70000000 - sizes.get("/").unwrap());
    let mut values: Vec<usize> = sizes.into_values().collect();
    values.sort();
    let mut candidate = 0;
    for v in values {
        candidate = v;
        if v > required_space {
            break;
        }
    }
    candidate.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
