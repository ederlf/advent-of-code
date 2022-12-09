use std::collections::HashSet;

fn chebyshev_distance(hh: (i32, i32), ht: (i32, i32)) -> i32 {
    let diff_x = i32::abs(hh.0 - ht.0);
    let diff_y = i32::abs(hh.1 - ht.1);
    i32::max(diff_x, diff_y)
}

fn find_point(hh: (i32, i32), ht: (i32, i32)) -> (i32, i32) {
    let filter_candidates = |x: &[(i32, i32)]| -> (i32, i32) {
        let mut neighbor = x.iter().filter(|x| chebyshev_distance(hh, **x) == 1);
        return *neighbor.next().unwrap();
    };

    if hh.1 == ht.1 {
        let candidates_x = [(ht.0 + 1, ht.1), (ht.0 - 1, ht.1)];
        return filter_candidates(&candidates_x);
    }

    if hh.0 == ht.0 {
        let candidates_y = [(ht.0, ht.1 + 1), (ht.0, ht.1 - 1)];
        return filter_candidates(&candidates_y);
    }

    let candidates = [
        (ht.0 + 1, ht.1 + 1),
        (ht.0 - 1, ht.1 - 1),
        (ht.0 + 1, ht.1 - 1),
        (ht.0 - 1, ht.1 + 1),
    ];

    filter_candidates(&candidates)
}

fn move_rope(
    knots: Vec<(i32, i32)>,
    direction: &str,
    times: usize,
    tail_visited: &mut HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let move_head = |h: (i32, i32), d| -> (i32, i32) {
        match d {
            "R" => (h.0, h.1 + 1),
            "L" => (h.0, h.1 - 1),
            "U" => (h.0 - 1, h.1),
            "D" => (h.0 + 1, h.1),
            _ => h,
        }
    };

    let move_tail = |hh: (i32, i32), ht: (i32, i32)| -> (i32, i32) {
        if chebyshev_distance(hh, ht) <= 1 {
            return ht;
        }
        find_point(hh, ht)
    };

    let mut new_positions = knots;
    for _ in 0..times {
        new_positions[0] = move_head(new_positions[0], direction);
        for idx in 1..new_positions.len() {
            new_positions[idx] = move_tail(new_positions[idx - 1], new_positions[idx]);
        }
        tail_visited.insert(*new_positions.last().unwrap());
    }

    new_positions
}

fn part1(input: String) -> String {
    let mut tail_visited = HashSet::new();
    let mut knots = vec![(0 as i32, 0 as i32), (0 as i32, 0 as i32)];
    tail_visited.insert(knots[0]);
    for movement in input.lines() {
        let mut m = movement.split(" ");
        let direction = m.next().unwrap();
        let times = m.next().unwrap().parse::<usize>().unwrap();
        knots = move_rope(knots, direction, times, &mut tail_visited);
    }

    let tail_visit_cnt = tail_visited.len();
    tail_visit_cnt.to_string()
}

fn part2(input: String) -> String {
    let mut tail_visited = HashSet::new();
    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push((0 as i32, 0 as i32));
    }
    tail_visited.insert(knots[0]);
    for movement in input.lines() {
        let mut m = movement.split(" ");
        let direction = m.next().unwrap();
        let times = m.next().unwrap().parse::<usize>().unwrap();
        knots = move_rope(knots, direction, times, &mut tail_visited);
    }

    let tail_visit_cnt = tail_visited.len();
    tail_visit_cnt.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
