use itertools::Itertools;

fn is_safe(report: &Vec<i32>) -> (usize, (usize, usize)) {
    let mut decreasing = false;
    let mut increasing = false;

    let mut x = 0;
    let mut y = 1;
    for (v1, v2) in report.iter().tuple_windows::<(&i32, &i32)>() {
        if (v2 - v1) == 0 {
            return (0, (x, y));
        }

        if (v2 - v1) > 0 {
            increasing = true;
        } else {
            decreasing = true;
        }

        if increasing && decreasing {
            return (0, (x, y));
        }

        if (v2 - v1).abs() > 3 || (v2 - v1).abs() < 1 {
            return (0, (x, y));
        }
        x+=1;
        y+=1;
    }
    return (1, (0, 0));
}

fn part1(input: String) -> String {
    let lines = input.trim().split("\n");
    let rows: Vec<Vec<i32>> = lines
        .map(|x| x.split_whitespace())
        .map(|x| x.map(|v| v.parse::<i32>().unwrap()).collect())
        .collect();
    let safe: usize = rows.into_iter().map(|x| is_safe(&x).0).sum();
    safe.to_string()
}

fn part2(input: String) -> String {
    let lines = input.trim().split("\n");
    let rows: Vec<Vec<i32>> = lines
        .map(|x| x.split_whitespace())
        .map(|x| x.map(|v| v.parse::<i32>().unwrap()).collect())
        .collect();
    let mut safe = 0;
    for row in rows.into_iter() {
        let safe_report = is_safe(&row);
        if safe_report.0 == 0 {
            for idx in 0..row.len() {
                let mut v = row.clone();
                v.remove(idx);
                if is_safe(&v).0 == 1 {
                    safe += 1;
                    break;
                }
            }
        } else {
            safe += 1;
        }

    }
    safe.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
