use std::collections::HashSet;

fn find_marker(input: String, block_size: usize) -> usize {
    let datastream: Vec<&str> = input.split("").filter(|x| !x.is_empty()).collect();
    let windows: Vec<HashSet<_>> = datastream
        .windows(block_size)
        .fold(Vec::new(), |mut acc, x| {
            let h = HashSet::from_iter(x.iter());
            acc.push(h);
            acc
        });

    let mut res = 0;
    for (i, w) in windows.iter().enumerate() {
        if w.len() == block_size {
            res += i + block_size;
            break;
        }
    }
    res
}

fn part1(input: String) -> String {
    find_marker(input, 4).to_string()
}

fn part2(input: String) -> String {
    find_marker(input, 14).to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
