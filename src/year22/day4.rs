use std::collections::HashSet;

fn pair_overlaps(line: &str, subset: bool) -> usize {
    let pairs: Vec<Vec<&str>> = line.split(",").map(|x| x.split("-").collect()).collect();
    let parse_int = |x: &Vec<&str>| -> Vec<_> {
        let parsed = x.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        parsed
    };
    let p1: Vec<_> = parse_int(&pairs[0]);
    let p2: Vec<_> = parse_int(&pairs[1]);

    let make_set = |x: Vec<usize>| {
        let set = x.chunks(2).fold(HashSet::new(), |mut acc, x| {
            acc.extend((x[0]..x[1] + 1).into_iter());
            acc
        });
        set
    };

    let h1 = make_set(p1);
    let h2 = make_set(p2);

    // part 1
    if subset {
        if h1.is_subset(&h2) || h2.is_subset(&h1) {
            return 1;
        } else {
            return 0;
        }
    }

    // part 2
    let inter: HashSet<_> = h1.intersection(&h2).collect();
    if inter.len() > 0 {
        1
    } else {
        0
    }
}

fn part1(input: String) -> String {
    let overlaps = input.lines().map(|x| pair_overlaps(x, true)).sum::<usize>();
    overlaps.to_string()
}

fn part2(input: String) -> String {
    let overlaps = input
        .lines()
        .map(|x| pair_overlaps(x, false))
        .sum::<usize>();
    overlaps.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
