use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub trait Movement {
    fn move_down(&self, n: i32) -> Self;
    fn move_up(&self, n: i32) -> Self;
    fn move_left(&self, n: i32) -> Self;
    fn move_right(&self, n: i32) -> Self;
    fn move_right_up(&self, n: i32) -> Self;
    fn move_left_up(&self, n: i32) -> Self;
}

impl Movement for Point {
    fn move_down(&self, n: i32) -> Self {
        Point {
            x: self.x - n,
            y: self.y,
        }
    }

    fn move_up(&self, n: i32) -> Self {
        Point {
            x: self.x + n,
            y: self.y,
        }
    }

    fn move_left(&self, n: i32) -> Self {
        Point {
            x: self.x,
            y: self.y - n,
        }
    }

    fn move_right(&self, n: i32) -> Self {
        Point {
            x: self.x,
            y: self.y + n,
        }
    }

    fn move_right_up(&self, n: i32) -> Self {
        Point {
            x: self.x + n,
            y: self.y + n,
        }
    }

    fn move_left_up(&self, n: i32) -> Self {
        Point {
            x: self.x + n,
            y: self.y - n,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ShapeType {
    HorizontalLine,
    VerticalLine,
    ReverseLShape,
    Cross,
    Square,
}

#[derive(Debug)]
struct Shape {
    start: Point,
    extension: i32,
    shape: ShapeType,
}

impl Shape {
    fn horizontal_line_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        points.insert(self.start);
        for i in 1..self.extension + 1 {
            points.insert(self.start.move_right(i));
        }
        points
    }

    fn vertical_line_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        points.insert(self.start);
        for i in 1..self.extension + 1 {
            points.insert(self.start.move_up(i));
        }
        points
    }

    fn reversel_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        points.insert(self.start);
        for i in 1..self.extension + 1 {
            points.insert(self.start.move_up(i));
            points.insert(self.start.move_left(i));
        }
        points
    }

    fn cross_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        points.insert(self.start);
        for i in 1..self.extension + 1 {
            points.insert(self.start.move_up(i));
            points.insert(self.start.move_left(i));
            points.insert(self.start.move_down(i));
            points.insert(self.start.move_right(i));
        }

        points
    }

    fn square_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        points.insert(self.start);
        for i in 1..self.extension + 1 {
            points.insert(self.start.move_up(i));
            let right = self.start.move_right(i);
            points.insert(right);
            points.insert(right.move_up(i));
        }

        points
    }

    fn points(&self) -> HashSet<Point> {
        match self.shape {
            ShapeType::HorizontalLine => self.horizontal_line_points(),
            ShapeType::VerticalLine => self.vertical_line_points(),
            ShapeType::ReverseLShape => self.reversel_points(),
            ShapeType::Cross => self.cross_points(),
            ShapeType::Square => self.square_points(),
        }
    }
}

fn start_shape(turn: usize, level: i32) -> Shape {
    let shape = turn % 5;
    match shape {
        0 => Shape {
            start: Point { x: level + 4, y: 2 },
            extension: 3,
            shape: ShapeType::HorizontalLine,
        },
        1 => Shape {
            start: Point { x: level + 5, y: 3 },
            extension: 1,
            shape: ShapeType::Cross,
        },
        2 => Shape {
            start: Point { x: level + 4, y: 4 },
            extension: 2,
            shape: ShapeType::ReverseLShape,
        },
        3 => Shape {
            start: Point { x: level + 4, y: 2 },
            extension: 3,
            shape: ShapeType::VerticalLine,
        },
        4 => Shape {
            start: Point { x: level + 4, y: 2 },
            extension: 1,
            shape: ShapeType::Square,
        },
        _ => panic!("Invalid value"),
    }
}

fn horizontal_line_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = shape.horizontal_line_points();
    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn horizontal_line_side_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = vec![shape.start, shape.start.move_right(3)];
    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn vertical_line_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    return grid.contains(&shape.start) || shape.start.x == 0;
}

fn vertical_line_side_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = shape.vertical_line_points();
    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn reversel_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = vec![
        shape.start,
        shape.start.move_left(1),
        shape.start.move_left(2),
    ];

    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn reversel_side_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = vec![
        shape.start,
        shape.start.move_up(1),
        shape.start.move_up(2),
        shape.start.move_left(2),
    ];

    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn cross_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = vec![
        shape.start.move_down(1),
        shape.start.move_right(1),
        shape.start.move_left(1),
    ];

    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn cross_side_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = shape.cross_points();

    for point in points {
        if grid.contains(&point) {
            return true;
        }
    }
    false
}

fn square_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = vec![shape.start, shape.start.move_right(1)];

    for point in points {
        if grid.contains(&point) || point.x == 0 {
            return true;
        }
    }
    false
}

fn square_side_touches(grid: &HashSet<Point>, shape: &Shape) -> bool {
    let points = shape.square_points();

    for point in points {
        if grid.contains(&point) {
            return true;
        }
    }
    false
}

fn shape_bottom_touches(grid: &HashSet<Point>, s: &Shape) -> bool {
    match s.shape {
        ShapeType::HorizontalLine => horizontal_line_touches(grid, s),
        ShapeType::VerticalLine => vertical_line_touches(grid, s),
        ShapeType::ReverseLShape => reversel_touches(grid, s),
        ShapeType::Cross => cross_touches(grid, s),
        ShapeType::Square => square_touches(grid, s),
    }
}

fn shape_side_touches(grid: &HashSet<Point>, s: &Shape) -> bool {
    match s.shape {
        ShapeType::HorizontalLine => horizontal_line_side_touches(grid, s),
        ShapeType::VerticalLine => vertical_line_side_touches(grid, s),
        ShapeType::ReverseLShape => reversel_side_touches(grid, s),
        ShapeType::Cross => cross_side_touches(grid, s),
        ShapeType::Square => square_side_touches(grid, s),
    }
}

fn out_of_bounds(s: &Shape) -> bool {
    match s.shape {
        ShapeType::HorizontalLine => s.start.move_right(3).y > 6 || s.start.y < 0,
        ShapeType::VerticalLine => s.start.y > 6 || s.start.y < 0,
        ShapeType::ReverseLShape => s.start.y > 6 || s.start.move_left(2).y < 0,
        ShapeType::Cross => s.start.move_right(1).y > 6 || s.start.move_left(1).y < 0,
        ShapeType::Square => s.start.move_right(1).y > 6 || s.start.y < 0,
    }
}

fn push_shape(side: char, shape: Shape, grid: &HashSet<Point>) -> Shape {
    let new_pos = match side {
        '>' => shape.start.move_right(1),
        '<' => shape.start.move_left(1),
        _ => panic!("Invalid direction token"),
    };

    let moved_shape = Shape {
        start: new_pos,
        extension: shape.extension,
        shape: shape.shape,
    };

    if out_of_bounds(&moved_shape) || shape_side_touches(grid, &moved_shape) {
        return shape;
    }

    moved_shape
}

fn move_shape_down(shape: Shape, grid: &HashSet<Point>) -> (Shape, bool) {
    let moved_shape = Shape {
        start: shape.start.move_down(1),
        extension: shape.extension,
        shape: shape.shape,
    };

    if out_of_bounds(&moved_shape) || shape_bottom_touches(grid, &moved_shape) {
        return (shape, false);
    }

    (moved_shape, true)
}

//fn print_grid(grid: &HashSet<Point>) {
//    for x in (0..20).rev() {
//        for y in 0..7 {
//            let point = Point { x, y };
//            if grid.contains(&point) {
//                print!("#");
//            } else {
//                print!(".");
//            }
//        }
//        print!("\n");
//    }
//}

fn part1(input: String) -> String {
    let directions: Vec<char> = input.chars().filter(|x| *x == '>' || *x == '<').collect();

    let mut grid: HashSet<Point> = HashSet::new();
    let mut level = 0;
    let mut rocks = 0;
    let mut full = 0;
    let mut shape = start_shape(rocks, level);
    let signs = directions.len() as i32;
    let mut sign_pos: i32 = -1;
    let mut prev = 0;
    let mut cycle_increases = Vec::new();
    for direction in directions.iter().cycle() {
        shape = push_shape(*direction, shape, &grid);
        let res = move_shape_down(shape, &grid);
        shape = res.0;
        if !res.1 {
            let new_points = shape.points();
            let shape_height = new_points.iter().map(|point| point.x).max().unwrap();
            grid.extend(shape.points());

            if shape_height > level {
                level = shape_height;
            }

            rocks += 1;

            sign_pos = (sign_pos + 1) % signs;
            if sign_pos == 0 && rocks % 5 == 1 {
                full += 1;
                if full == 17 {
                    break;
                }
                //println!(
                //   "{:?} {} {} level {} {}",
                //  shape,
                // sign_pos,
                //rocks,
                //level,
                //level - prev
                //);
                cycle_increases.push((rocks as i64, (level - prev) as i64));
                prev = level;
            }

            shape = start_shape(rocks, level);
        }
    }

    println!("Calculating height {:?}", cycle_increases);
    let shapes_per_cycle: i64 = cycle_increases.iter().skip(1).skip(2).map(|x| x.0).sum();
    let growth_per_cycle: i64 = cycle_increases.iter().skip(1).skip(2).map(|x| x.1).sum();
    let target: i64 = 1000000000000 - cycle_increases[0].0;
    let mut remainder = target % shapes_per_cycle;
    let total_cycles = (target - remainder) / shapes_per_cycle;
    println!(
        "Cycles {} remainder {} growth {}",
        total_cycles, remainder, growth_per_cycle
    );
    let mut height = cycle_increases[0].1 + (total_cycles * growth_per_cycle);
    for v in cycle_increases.iter().skip(1) {
        height += v.1;
        remainder -= v.0;
        if remainder <= 0 {
            break;
        }
    }

    height.to_string()
}

fn part2(_input: String) -> String {
    "Not implemented".to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
