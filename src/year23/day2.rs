#[derive(Debug)]
struct GameSet {
    blue: i64,
    green: i64,
    red: i64,
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

fn parse_game_set(game_set: &str) -> GameSet {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    let colors = game_set.split(',');
    for color in colors {
        let mut color_quantity = color.trim().split(' ');
        let qt = color_quantity.next().unwrap().parse::<i64>().unwrap();
        let c = color_quantity.next().unwrap();
        match c {
            "blue" => blue += qt,
            "red" => red += qt,
            "green" => green += qt,
            _ => (),
        };
    }

    let set = GameSet { blue, green, red };
    set
}

fn parse_game(line: &str) -> Game {
    let mut game_sets = line.split(':');
    // It's a well defined input... but all this wrapping
    // can't put anyone at ease...
    let id: i32 = game_sets
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let sets: Vec<GameSet> = game_sets
        .next()
        .unwrap()
        .split(';')
        .map(|x| parse_game_set(x))
        .collect();

    let game = Game { id, sets };
    game
}

fn is_possible(game: &Game) -> i32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    for set in &game.sets {
        if set.blue > max_blue || set.green > max_green || set.red > max_red {
            return 0;
        }
    }
    game.id
}

fn minimum_set(game: &Game) -> i64 {
    let blue = &game.sets.iter().map(|x| x.blue).max().unwrap();
    let green = &game.sets.iter().map(|x| x.green).max().unwrap();
    let red = &game.sets.iter().map(|x| x.red).max().unwrap();
    blue * green * red
}

fn part1(input: String) -> String {
    let games: Vec<Game> = input.lines().map(|x| parse_game(x)).collect();
    let possible: i32 = games.iter().map(|x| is_possible(x)).sum();
    possible.to_string()
}

fn part2(input: String) -> String {
    let games: Vec<Game> = input.lines().map(|x| parse_game(x)).collect();
    let power_set: i64 = games.iter().map(|x| minimum_set(x)).sum();
    power_set.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
