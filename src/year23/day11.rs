use itertools::Itertools;
use pathfinding::prelude::Grid;
use std::collections::HashMap;
use std::collections::HashSet;

fn print_grid(grid: &HashMap<(usize, usize), char>, rows: usize, cols: usize) {
    for row in 0..rows {
        for col in 0..cols {
            print!("{:?}", grid.get(&(row, col)).unwrap());
        }
        println!("\n");
    }
}

fn day11(input: String, expand_by: usize) -> String {
    let mut cols = 0;
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        cols = line.len();
        for v in line.trim().chars() {
            row.push(v);
        }
        rows.push(row);
        if !line.contains('#') {
            let mut row: Vec<char> = Vec::new();
            row.push('.');
            for _ in 0..expand_by - 1 {
                rows.push(row.clone());
            }
        }
    }
    let mut expand_cols = HashSet::new();
    for i in 0..cols {
        let mut all_dots = true;
        for row in rows.iter() {
            if row.len() != 1 {
                if row[i] != '.' {
                    all_dots = false;
                    break;
                }
            }
        }
        if all_dots {
            expand_cols.insert(i);
        }
    }

    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut r: usize = 0;
    for row in rows {
        let mut offset = 0;
        for (i, col) in row.iter().enumerate() {
            grid.insert((i + offset, r), *col);
            if expand_cols.contains(&i) {
                offset += expand_by - 1;
            }
        }
        r += 1;
    }

    grid.retain(|_, v| *v == '#');

    let g = grid.keys().into_iter().map(|x| *x).collect::<Grid>();
    let total: usize = grid
        .keys()
        .combinations(2)
        .map(|v| g.distance(*v[0], *v[1]))
        .sum();
    //for (i, v) in grid.keys().combinations(2).enumerate() {
    //        println!("{} {:?} {:?}", i, *v[0], *v[1]);
    //       println!("{:?}", g.distance(*v[0], *v[1]));
    // }
    //println!("{:?}", g);
    //print_grid(&grid, r, cols);
    total.to_string()
}

fn part1(input: String) -> String {
    day11(input, 2)
}

fn part2(input: String) -> String {
    day11(input, 1000000)
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
