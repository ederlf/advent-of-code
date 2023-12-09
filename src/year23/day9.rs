fn part1(input: String) -> String {
    let sequences = input.lines().map(|x| x.trim());
    let mut last_values: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    for seq in sequences {
        let start_seq: Vec<i32> = seq.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        last_values.push(*start_seq.last().unwrap());
        let mut diffs: Vec<i32> = start_seq.windows(2).map(|x| x[1] - x[0]).collect();
        while diffs.iter().sum::<i32>() != 0 {
            last_values.push(*diffs.last().unwrap());
            diffs = diffs.windows(2).map(|x| x[1] - x[0]).collect();
        }

        let mut next: i32 = 0;
        for v in last_values.iter().rev() {
            next = next + *v;
        }
        total += next;
        last_values.clear();
    }
    total.to_string()
}

fn part2(input: String) -> String {
    let sequences = input.lines().map(|x| x.trim());
    let mut last_values: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    for seq in sequences {
        let start_seq: Vec<i32> = seq.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        last_values.push(*start_seq.first().unwrap());
        let mut diffs: Vec<i32> = start_seq.windows(2).map(|x| x[1] - x[0]).collect();
        while diffs.iter().sum::<i32>() != 0 {
            last_values.push(*diffs.first().unwrap());
            diffs = diffs.windows(2).map(|x| x[1] - x[0]).collect();
        }

        let mut next: i32 = 0;
        for v in last_values.iter().rev() {
            next = *v - next;
        }
        total += next;
        last_values.clear();
    }
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
