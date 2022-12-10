use std::collections::HashSet;

fn sprite_overlaps(x: i32, cycle: i32) -> bool {
    let mut h = HashSet::new();
    h.insert(x);
    h.insert(x - 1);
    h.insert(x + 1);
    h.contains(&cycle)
}

fn simulate(input: String) -> (String, String) {
    let mut instructions = input.lines().map(|x| x.split(" ").collect::<Vec<&str>>());
    let mut x = 1;
    let mut cycle = 0;
    let mut next = (0, 0);
    let mut strength = Vec::new();

    let mut crt: Vec<Vec<&str>> = Vec::new();
    for _ in 0..6 {
        crt.push(Vec::new());
    }

    let mut hline = 0;
    loop {
        cycle += 1;
        if cycle > 240 {
            break;
        }

        // part 1
        if cycle == 20
            || cycle == 60
            || cycle == 100
            || cycle == 140
            || cycle == 180
            || cycle == 220
        {
            strength.push(x * cycle);
        }

        if sprite_overlaps(x, (cycle - 1) % 40) {
            crt[hline].push("#");
        } else {
            crt[hline].push(".");
        }

        if cycle % 40 == 0 {
            hline += 1;
        }

        if next == (0, 0) {
            let next_inst = instructions.next();
            if next_inst == None {
                break;
            }

            let inst = next_inst.unwrap();
            let op = inst[0];
            if op == "addx" {
                let v = inst[1].parse::<i32>().unwrap();
                next = (cycle + 1, v);
            }
        }

        if next.0 == cycle {
            x += next.1;
            next = (0, 0);
        }
    }

    let letters = crt
        .iter()
        .map(|x| x.join(""))
        .collect::<Vec<String>>()
        .join("\n");

    let total = strength.iter().sum::<i32>();
    (total.to_string(), format!("\n{}", letters))
}

fn part1(input: String) -> String {
    simulate(input).0
}

fn part2(input: String) -> String {
    simulate(input).1
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
