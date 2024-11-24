use itertools::Itertools;

#[derive(Debug)]
struct Dimension {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimension {
    fn area(&self) -> u32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
    }

    fn smallest_side(&self) -> u32 {
        let v = vec![
            self.length * self.width,
            self.width * self.height,
            self.length * self.height,
        ];
        *v.iter().min().unwrap()
    }

    fn perimiter(&self) -> u32 {
        let v: Vec<u32> = vec![self.length, self.width, self.height]
            .into_iter()
            .sorted()
            .collect();
        2 * v[0] + 2 * v[1]
    }

    fn cubic(&self) -> u32 {
        self.height * self.width * self.length
    }
}

fn dimensions(input: String) -> Vec<Dimension> {
    let d: Vec<Vec<u32>> = input
        .trim()
        .split("\n")
        .map(|x| x.split("x").map(|v| v.parse::<u32>().unwrap()).collect())
        .collect();
    let dimensions = d
        .into_iter()
        .map(|x| {
            x.into_iter().enumerate().fold(
                Dimension {
                    length: 0,
                    width: 0,
                    height: 0,
                },
                |mut acc, (index, value)| {
                    match index {
                        0 => acc.length = value,
                        1 => acc.width = value,
                        2 => acc.height = value,
                        _ => (),
                    }
                    acc
                },
            )
        })
        .collect();
    dimensions
}

fn part1(input: String) -> String {
    let dimensions = dimensions(input);
    let res: u32 = dimensions
        .into_iter()
        .map(|x| x.area() + x.smallest_side())
        .sum();
    res.to_string()
}

fn part2(input: String) -> String {
    let dimensions = dimensions(input);
    let res: u32 = dimensions
        .into_iter()
        .map(|x| x.perimiter() + x.cubic())
        .sum();
    res.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
