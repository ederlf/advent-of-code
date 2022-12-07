use curl::easy::Easy;
use std::fs;

pub fn input_file_name(year: u32, day: u32, test: bool) -> String {
    format!(
        "{}/{}-{}.txt",
        if !test { "inputs" } else { "test_inputs" },
        year,
        day
    )
}

pub fn download_input(year: u32, day: u32, cookie: &str) -> String {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut easy = Easy::new();
    easy.cookie(cookie).expect("Failed to set URL cookie");
    easy.url(&url).unwrap();
    easy.write_function(move |data| {
        let write_result = fs::write(input_file_name(year, day, false), data);
        let _ = match write_result {
            Ok(bytes) => bytes,
            Err(error) => panic!("There was a problem saving the file: {:?}", error),
        };
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
    fs::read_to_string(input_file_name(year, day, false))
        .expect("Could not read file after download")
}

pub fn parse_number_lines(input: String) -> Vec<i32> {
    let parse = |x: &str| -> i32 { x.parse::<i32>().expect("Parse error") };
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(parse)
        .collect()
}
