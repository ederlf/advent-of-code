use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: String,
    op_factor: u64,
    test_div: u64,
    tresult: u64,
    fresult: u64,
    checked: u64,
}

impl Monkey {
    fn new(desc: &str) -> Self {
        let mut monkey = Monkey {
            items: VecDeque::new(),
            op: String::new(),
            op_factor: 0,
            test_div: 0,
            tresult: 0,
            fresult: 0,
            checked: 0,
        };

        for line in desc.lines() {
            if line.contains("Starting") {
                let items = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .map(|x| x.trim().parse::<u64>().unwrap());
                monkey.items = items.collect();
            } else if line.contains("Operation") {
                let operation = line
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>();
                monkey.op = operation[1].to_string();
                if operation[2] == "old" {
                    monkey.op_factor = 1;
                } else {
                    monkey.op_factor = operation[2].parse::<u64>().unwrap();
                }
            } else if line.contains("Test") {
                monkey.test_div = line.split(" ").last().unwrap().parse::<u64>().unwrap();
            } else if line.contains("true") {
                monkey.tresult = line.split(" ").last().unwrap().parse::<u64>().unwrap();
            } else if line.contains("false") {
                monkey.fresult = line.split(" ").last().unwrap().parse::<u64>().unwrap();
            }
        }
        monkey
    }

    // Returns the id of the next monkey
    fn inspect_item(&mut self, item: u64, divide: u64, modprod: u64, part2: bool) -> (u64, u64) {
        self.checked += 1;
        let factor = if self.op_factor == 1 {
            item
        } else {
            self.op_factor
        };

        let mut worry_level = match self.op.as_str() {
            "*" => (item * factor) / divide,
            "+" => (item + factor) / divide,
            _ => item,
        };

        if part2 {
            worry_level = worry_level % modprod;
        }

        // throw to
        let next_monkey = match worry_level % self.test_div == 0 {
            false => self.fresult,
            true => self.tresult,
        };

        (worry_level, next_monkey)
    }

    fn receive_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn part1(input: String) -> String {
    let mut monkeys = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| Monkey::new(x))
        .collect::<Vec<Monkey>>();

    let mut turns = 0;
    loop {
        turns += 1;
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop_front().unwrap();
                let (worry_level, throw_to) = monkeys[i].inspect_item(item, 3, 0, false);
                monkeys[throw_to as usize].receive_item(worry_level);
            }
        }
        if turns == 20 {
            break;
        }
    }
    let mut checks = monkeys.iter().map(|x| x.checked).collect::<Vec<u64>>();
    checks.sort();
    checks.reverse();

    (checks[0] * checks[1]).to_string()
}

fn part2(input: String) -> String {
    let mut monkeys = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| Monkey::new(x))
        .collect::<Vec<Monkey>>();

    let mut turns = 0;
    let modprod = monkeys.iter().fold(1, |acc, x| acc * x.test_div);
    println!("{}", modprod);
    loop {
        turns += 1;
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop_front().unwrap();
                let (worry_level, throw_to) = monkeys[i].inspect_item(item, 1, modprod, true);
                monkeys[throw_to as usize].receive_item(worry_level);
            }
        }
        if turns == 10000 {
            break;
        }
    }
    let mut checks = monkeys.iter().map(|x| x.checked).collect::<Vec<u64>>();
    checks.sort();
    checks.reverse();

    (checks[0] * checks[1]).to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
