use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Number {
    n: usize,
    coordinates: Vec<(i32, i32)>,
}

fn has_symbol(grid: &HashMap<(i32, i32), char>, coord: &(i32, i32)) -> (bool, bool) {
    if !&grid.contains_key(coord) {
        return (false, false);
    }

    let v = *grid.get(coord).unwrap();
    if v == '.' || v.is_digit(10) {
        return (false, false);
    }
    if v == '*' {
        return (true, true);
    }

    return (true, false);
}

fn neighbors(coord: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut coordinates = Vec::new();
    // up
    coordinates.push((coord.0 - 1, coord.1));
    // down
    coordinates.push((coord.0 + 1, coord.1));
    // left
    coordinates.push((coord.0, coord.1 - 1));
    // right
    coordinates.push((coord.0, coord.1 + 1));
    // up left
    coordinates.push((coord.0 - 1, coord.1 - 1));
    // up right
    coordinates.push((coord.0 - 1, coord.1 + 1));
    // down left
    coordinates.push((coord.0 + 1, coord.1 - 1));
    // down right
    coordinates.push((coord.0 + 1, coord.1 + 1));
    coordinates
}

fn has_neighbor_symbol(grid: &HashMap<(i32, i32), char>, n: &Number) -> usize {
    for coord in &n.coordinates {
        let neighbors = neighbors(coord);
        let contain_symbol: Vec<(i32, i32)> = neighbors
            .into_iter()
            .filter(|x| has_symbol(grid, x).0)
            .collect();
        if !contain_symbol.is_empty() {
            return n.n;
        }
    }
    0
}

fn has_neighbor_asterisk(
    grid: &HashMap<(i32, i32), char>,
    n: &Number,
) -> (usize, HashSet<(i32, i32)>) {
    let mut contain_asterisk: HashSet<(i32, i32)> = HashSet::new();
    for coord in &n.coordinates {
        let neighbors = neighbors(coord);
        let contain_symbol: Vec<(i32, i32)> = neighbors
            .into_iter()
            .filter(|x| has_symbol(grid, x).1)
            .collect();
        contain_asterisk.extend(contain_symbol);
    }
    (n.n, contain_asterisk)
}

fn part1(input: String) -> String {
    let mut grid = HashMap::new();
    let mut row = 0;
    //    let mut max_col = 0;
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut col = 0;
        let mut num = Vec::new();
        let mut num_coord = Vec::new();
        for c in line.trim().chars() {
            grid.insert((row, col), c);
            if c.is_digit(10) {
                num.push(c);
                num_coord.push((row, col));
            } else {
                if !num.is_empty() {
                    let n = num.iter().collect::<String>().parse::<usize>().unwrap();
                    numbers.push(Number {
                        n,
                        coordinates: num_coord.clone(),
                    });
                    num.clear();
                    num_coord.clear();
                }
            }
            col += 1;
        }
        if !num.is_empty() {
            let n = num.iter().collect::<String>().parse::<usize>().unwrap();
            numbers.push(Number {
                n,
                coordinates: num_coord.clone(),
            });
            num.clear();
            num_coord.clear();
        }
        row += 1;
    }
    let res: usize = numbers.iter().map(|x| has_neighbor_symbol(&grid, x)).sum();
    res.to_string()
}

fn part2(input: String) -> String {
    let mut grid = HashMap::new();
    let mut row = 0;
    //    let mut max_col = 0;
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut col = 0;
        let mut num = Vec::new();
        let mut num_coord = Vec::new();
        for c in line.trim().chars() {
            grid.insert((row, col), c);
            if c.is_digit(10) {
                num.push(c);
                num_coord.push((row, col));
            } else {
                if !num.is_empty() {
                    let n = num.iter().collect::<String>().parse::<usize>().unwrap();
                    numbers.push(Number {
                        n,
                        coordinates: num_coord.clone(),
                    });
                    num.clear();
                    num_coord.clear();
                }
            }
            col += 1;
        }
        if !num.is_empty() {
            let n = num.iter().collect::<String>().parse::<usize>().unwrap();
            numbers.push(Number {
                n,
                coordinates: num_coord.clone(),
            });
            num.clear();
            num_coord.clear();
        }
        row += 1;
    }
    let res: Vec<(usize, HashSet<(i32, i32)>)> = numbers
        .iter()
        .map(|x| has_neighbor_asterisk(&grid, x))
        .collect();

    let mut count: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (n, coordinates) in res {
        for c in coordinates {
            if !count.contains_key(&c) {
                count.insert(c, Vec::new());
            }
            count.get_mut(&c).unwrap().push(n);
        }
    }
    let mut total = 0;
    for v in count.values() {
        if v.len() > 1 {
            total += v.into_iter().copied().reduce(|acc, e| acc * e).unwrap();
        }
    }
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
