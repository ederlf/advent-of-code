fn part1(input: String)  -> String {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in input.split("\n").filter(|x| !x.is_empty()){
        let directions: Vec<&str> = line.split(" ").collect();
        let units = directions[1].parse::<i32>().expect("Failed to parse units");
        match directions[0] {
            "forward" => horizontal += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => panic!("Direction not allowed"),

        };
    }
    let total = depth * horizontal;
    total.to_string()
}

fn part2(input: String) -> String {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in input.split("\n").filter(|x| !x.is_empty()){
        let directions: Vec<&str> = line.split(" ").collect();
        let units = directions[1].parse::<i32>().expect("Failed to parse units");
        match directions[0] {
            "forward" => {
                horizontal += units;
                depth += aim * units;
            },
            "down" => aim += units,
            "up" => aim -= units,
            _ => panic!("Direction not allowed"),

        };
    }
    let total = depth * horizontal;
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
