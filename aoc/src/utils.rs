use std::fs;
use curl::easy::Easy;

pub fn input_file_name(year: u32, day: u32) -> String {
    format!("inputs/{}-{}.txt", year, day)
}

pub fn download_input(year: u32, day: u32, cookie: &str) -> String {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut easy = Easy::new();
    easy.cookie(cookie).expect("Failed to set URL cookie");
    easy.url(&url).unwrap();
    easy.write_function(move |data| {
        let write_result = fs::write(input_file_name(year, day), data);
        let _ = match write_result {
            Ok(bytes) => bytes,
            Err(error) => panic!("There was a problem saving the file: {:?}", error),
        };
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    fs::read_to_string(input_file_name(year, day)).expect("Could not read file after download")
}

pub fn parse_number_lines(input: String) -> Vec<i32> {
    let parse = |x: &str| -> i32 {
        x.parse::<i32>().expect("Parse error")
    };
    input.split("\n").filter(|x| !x.is_empty()).map(parse).collect()
}
