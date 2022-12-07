use std::collections::HashMap;

fn parse_point(point: &str) -> (usize, usize) {
    let mut iter = point.split(",");
    let (x1, y1) = (iter.next().unwrap(), iter.next().unwrap());
    (x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap())
}

fn find_intersections(
    points: &mut HashMap<(usize, usize), usize>,
    p1: (usize, usize),
    p2: (usize, usize),
) {
    let diff_x = p1.0 as i32 - p2.0 as i32;
    let diff_y = p1.1 as i32 - p2.1 as i32;

    let diff = {
        if diff_x != 0 {
            diff_x
        } else {
            diff_y
        }
    };

    for i in 0..diff.abs() as usize + 1 {
        let mut point = (0, 0);
        if diff_x < 0 && diff_y < 0 {
            point = (p2.0 - i, p2.1 - i);
        } else if diff_x > 0 && diff_y < 0 {
            point = (p2.0 + i, p2.1 - i);
        } else if diff_x < 0 && diff_y > 0 {
            point = (p2.0 - i, p2.1 + i);
        } else if diff_x > 0 && diff_y > 0 {
            point = (p2.0 + i, p2.1 + i);
        } else if diff_x == 0 {
            if diff_y < 0 {
                point = (p1.0, p2.1 - i);
            } else {
                point = (p1.0, p2.1 + i);
            }
        } else if diff_y == 0 {
            if diff_x < 0 {
                point = (p2.0 - i, p1.1);
            } else {
                point = (p2.0 + i, p1.1);
            }
        }

        if points.contains_key(&point) {
            points.insert(point, points[&point] + 1);
        } else {
            points.insert(point, 0);
        }
    }
}

fn part1(input: String) -> String {
    let coordinates = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(" "));
    let mut points = HashMap::new();
    for coord in coordinates {
        let c = coord.collect::<Vec<_>>();
        let (x1, y1) = parse_point(c[0]);
        let (x2, y2) = parse_point(c[2]);
        if x1 == x2 || y1 == y2 {
            find_intersections(&mut points, (x1, y1), (x2, y2));
        }
    }
    let total = points.into_values().filter(|x| *x > 0).count();
    total.to_string()
}

fn part2(input: String) -> String {
    let coordinates = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(" "));
    let mut points = HashMap::new();
    for coord in coordinates {
        let c = coord.collect::<Vec<_>>();
        let (x1, y1) = parse_point(c[0]);
        let (x2, y2) = parse_point(c[2]);
        find_intersections(&mut points, (x1, y1), (x2, y2));
    }
    let total = points.into_values().filter(|x| *x > 0).count();
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
