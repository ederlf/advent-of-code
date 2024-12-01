mod utils;
mod year15;
mod year21;
mod year22;
mod year23;
mod year24;
use std::fs;

use clap::Parser;

/// Solves a specific day of Advent of Code (AOC)
#[derive(Parser)]
struct Cli {
    /// The year of the AOC puzzle
    year: u32,
    /// The day of the AOC puzzle
    day: u32,
    /// Use a test input
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

fn main() {
    let args = Cli::parse();
    let cookie = env!("AOC_SESSION_COOKIE");

    let input_file_result =
        fs::read_to_string(utils::input_file_name(args.year, args.day, args.test));
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(_) => utils::download_input(args.year, args.day, cookie),
    };

    let result = match args.year {
        2015 => year15::solve(args.day, input_file),
        2021 => year21::solve(args.day, input_file),
        2022 => year22::solve(args.day, input_file),
        2023 => year23::solve(args.day, input_file),
        2024 => year24::solve(args.day, input_file),
        _ => panic!("Year {} does not exist", args.year),
    };

    println!(
        "Result for {}/{} is: Part1 {}, Part 2 {}",
        args.year, args.day, result.0, result.1
    )
}
