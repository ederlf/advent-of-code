use std::collections::HashMap;

fn is_visible(x: i32, y: i32, last: usize, grid: &HashMap<(i32, i32), u32>) -> (usize, usize) {
    let point = (x, y);
    let up = (x, y + 1);
    let down = (x, y - 1);
    let left = (x - 1, y);
    let right = (x + 1, y);

    if !grid.contains_key(&up)
        || !grid.contains_key(&down)
        || !grid.contains_key(&left)
        || !grid.contains_key(&right)
    {
        return (1, 0);
    }

    // Check down
    let mut blocked = 0;
    let mut down_score = 0;
    for v in x + 1..last as i32 {
        let down = (v, y);
        down_score += 1;
        if grid[&point] <= grid[&down] {
            blocked += 1;
            break;
        }
    }

    let mut up_score = 0;
    for v in (0..x).rev() {
        let up = (v, y);
        up_score += 1;
        if grid[&point] <= grid[&up] {
            blocked += 1;
            break;
        }
    }

    let mut left_score = 0;
    for v in (0..y).rev() {
        let left = (x, v);
        left_score += 1;
        if grid[&point] <= grid[&left] {
            blocked += 1;
            break;
        }
    }

    let mut right_score = 0;
    for v in y + 1..last as i32 {
        let right = (x, v);
        right_score += 1;
        if grid[&point] <= grid[&right] {
            blocked += 1;
            break;
        }
    }

    let score = up_score * down_score * right_score * left_score;
    if blocked == 4 {
        return (0, score);
    }
    (1, score)
}

fn tree_top(input: String, part1: bool) -> String {
    let grid_lines: Vec<_> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let last = grid_lines[0].len();
    let mut grid = HashMap::new();
    for x in 0..last {
        for y in 0..last {
            grid.insert((x as i32, y as i32), grid_lines[x][y]);
        }
    }

    let mut visible_count = 0;
    let mut total = 0;
    for x in 0..last {
        for y in 0..last {
            let (visible, score) = is_visible(x as i32, y as i32, last, &grid);
            visible_count += visible;
            if score > total {
                total = score;
            }
        }
    }

    if part1 {
        return visible_count.to_string();
    }

    total.to_string()
}

fn part1(input: String) -> String {
    tree_top(input, true)
}

fn part2(input: String) -> String {
    tree_top(input, false)
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
