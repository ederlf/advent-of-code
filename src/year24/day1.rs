use itertools::Itertools;
use std::iter::zip;
use pathfinding::num_traits::abs;

fn part1(input: String)  -> String {
    let list = input.trim()
                .split("\n")
                .map(|x| x.split_whitespace());
    let int_list: Vec<_> = list.map(|x| x.map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let right_list = int_list.iter().map(|x| x[0]).sorted();
    let left_list = int_list.iter().map(|x| x[1]).sorted();
    let res: i32 = zip(right_list, left_list).map(|x| abs(x.0 - x.1)).sum();
    res.to_string()
}

fn part2(input: String) -> String {
    let list = input.trim()
                .split("\n")
                .map(|x| x.split_whitespace());
    let int_list: Vec<_> = list.map(|x| x.map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();
    let right_list = int_list.iter().map(|x| x[0]).sorted();
    let left_list = int_list.iter().map(|x| x[1]).sorted();
    let mut res = 0;
    // This is innefficient but the input is small and the program short lived :shrugs:
    for ri in right_list  {
        res += left_list.clone().filter(|x| *x == ri).count() * ri;
    }
    res.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
