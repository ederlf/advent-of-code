use std::collections::HashMap;

#[derive(Debug)]
struct Fish {
    counter: i64,
}

fn part1(input: String) -> String {
    let mut fish: Vec<Fish> = input
        .trim_end_matches(&['\r', '\n'][..])
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| Fish {
            counter: x.parse::<i64>().unwrap(),
        })
        .collect();
    for _ in 0..80 {
        let mut new_fish = Vec::new();
        for mut f in fish.iter_mut() {
            f.counter -= 1;
            if f.counter < 0 {
                f.counter = 6;
                new_fish.push(Fish { counter: 8 });
            }
        }
        fish.append(&mut new_fish);
    }
    fish.len().to_string()
}

fn calculate(init: i64, day: i64, total: i64, cache: &mut HashMap<i64, HashMap<i64, i64>>) -> i64 {
    let rem_days = total - day;
    if rem_days <= init {
        return 0;
    }

    if cache.get(&day).unwrap().contains_key(&init) {
        let will_generate = *cache.get(&day).unwrap().get(&init).unwrap();
        return will_generate;
    }

    let mut will_generate = 1 + (rem_days - (init + 1)) / 7;
    let mut next_day = day + init + 1;
    for _ in 0..will_generate {
        will_generate += calculate(8, next_day, total, cache);
        next_day += 7;
    }

    cache.get_mut(&day).unwrap().insert(init, will_generate);
    will_generate
}

fn part2(input: String) -> String {
    //let input = "3,4,3,1,2";
    let mut fish: Vec<Fish> = input
        .trim_end_matches(&['\r', '\n'][..])
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| Fish {
            counter: x.parse::<i64>().unwrap(),
        })
        .collect();
    let mut cache = HashMap::new();
    for i in 0..256 {
        cache.insert(i, HashMap::new());
    }
    let mut total = 0;
    for f in fish.iter_mut() {
        total += 1 + calculate(f.counter, 0, 256, &mut cache);
    }
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
