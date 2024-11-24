use std::collections::HashSet;

use itertools::Itertools;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn next_point(direction: char, current_point: Point) -> Point{
    let next = match direction {
        '>' => Point {
            x: current_point.x+ 1,
            y: current_point.y,
        },
        '<' => Point {
            x: current_point.x - 1,
            y: current_point.y,
        },
        '^' => Point {
            x: current_point.x,
            y: current_point.y + 1,
        },
        'v' => Point {
            x: current_point.x,
            y: current_point.y - 1,
        },
        _ => panic!("Invalid direction"),
    };
    next
}
fn part1(input: String) -> String {
    let mut p: Point = Point { x: 0, y: 0 };
    let mut points: HashSet<Point> = HashSet::new();
    points.insert(p.clone());
    for direction in input.trim().chars() {
        let next_point = next_point(direction, p);         
        points.insert(next_point.clone());
        p = next_point;
    }
    points.len().to_string()
}


fn part2(input: String) -> String {
    let mut santa: Point = Point { x: 0, y: 0 };
    let mut robot: Point = Point { x: 0, y: 0 };
    let mut points: HashSet<Point> = HashSet::new();
    points.insert(santa.clone());
    for (santa_direction, robot_direction) in input.trim().chars().into_iter().tuples() {
        let next_santa = next_point(santa_direction, santa);
        points.insert(next_santa.clone());
        santa = next_santa;
        let next_robot = next_point(robot_direction, robot);
        points.insert(next_robot.clone());
        robot = next_robot;
    }
    points.len().to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
