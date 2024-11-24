use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use itertools::Itertools;

fn part1(input: String) -> String {
    let res: i32 = input
        .into_bytes()
        .iter()
        .map(|x| match x {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum();
    res.to_string()
}

fn part2(input: String) -> String {
    // I just wanted to use fold_while :) 
    let res = input
        .into_bytes()
        .iter()
        .enumerate()
        .fold_while((0, 0), |(acc, idx), (i, x)| {
            if acc == -1 {
                Done((acc, idx))
            } else {
                Continue((
                    acc + {
                        match x {
                            b'(' => 1,
                            b')' => -1,
                            _ => 0,
                        }
                    },
                    i+1,
                ))
            }
        })
        .into_inner();
    res.1.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
