use std::collections::HashSet;

fn fold(v: usize, fold_at: usize) -> usize {
    if v > fold_at {
        return fold_at - (v - fold_at);
    }   
    v
}

fn part1(input: String)  -> String {
    let coordinates = input.split("\n").filter(|x| x.contains(","));
    let instruction = input.split("\n").filter(|x| x.contains("fold")).collect::<Vec<&str>>()[0];
    let mut grid = HashSet::new();
    for coord in coordinates {
        let xy: Vec<usize> = coord.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        if instruction.contains("y=") {
            let fold_at = instruction.split("y=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            grid.insert((xy[0], fold(xy[1], fold_at)));
        }
        if instruction.contains("x=") {
            let fold_at = instruction.split("x=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            grid.insert((fold(xy[0], fold_at), xy[1]));
        }
    }
    grid.len().to_string()
}

fn print_grid(grid: &HashSet<(usize, usize)>) {

    for y in 0..6 {
        for x in 0..40 {
           if grid.contains(&(x, y)) {
                print!("#");
           } else {
            print!(".");
          }
        }
        print!("\n");
    }

}

fn part2(input: String) -> String {
    let coordinates = input.split("\n").filter(|x| x.contains(","));
    let mut instructions: Vec<&str> = input.split("\n").filter(|x| x.contains("fold")).collect();
    let instruction = instructions[0];
    let mut grid = HashSet::new();
    for coord in coordinates {
        let xy: Vec<usize> = coord.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let fold_at = match instruction.contains("y=") {
            true => instruction.split("y=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
            false => instruction.split("x=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
        };
        if instruction.contains("y=") {
            grid.insert((xy[0], fold(xy[1], fold_at)));
        }
        if instruction.contains("x=") {
            grid.insert((fold(xy[0], fold_at), xy[1]));
        }
    }
    instructions.remove(0);

    let mut new_coordinates = grid.clone();
    for instruction in instructions {
        let fold_at = match instruction.contains("y=") {
            true => instruction.split("y=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
            false => instruction.split("x=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
        };

        for coord in new_coordinates.iter() {
            grid.remove(coord);
            if instruction.contains("y=") {
                grid.insert((coord.0, fold(coord.1, fold_at)));
            }
            if instruction.contains("x=") {
                grid.insert((fold(coord.0, fold_at), coord.1));
            }
        }
        new_coordinates = grid.clone();
    }

    print_grid(&grid);
    grid.len().to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
