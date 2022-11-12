mod utils;
use std::fs;

use clap::Parser;

/// Solves a specific day of Advent of Code (AOC)
#[derive(Parser)]
struct Cli {
    /// The year of the AOC puzzle
    year: u32,
    /// The day of the AOC puzzle
    day: u32,
}


fn main() {
    let args = Cli::parse();
    let cookie = env!("AOC_SESSION_COOKIE");

    let input_file_result = fs::read_to_string(utils::input_file_name(args.year, args.day));
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(_) => utils::download_input(args.year, args.day, cookie),
    };
}