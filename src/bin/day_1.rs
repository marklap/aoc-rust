use std::env;

use std::io::Error as IoError;
use std::path::Path;

const INPUT_FILE_PATH: &str = "src/bin/day_1_input.txt";
// const INPUT_FILE_PATH: &str = "src/bin/day_1_part_2_calib.txt";

const DECIMALS: u32 = 10;
type WordMap = [(&'static str, u32); 9];

fn find_digit_l2r(map: WordMap, line: &String) -> Option<u32> {
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(DECIMALS);
        }
        for (w, d) in map {
            if line[i..].starts_with(w) {
                return Some(d);
            }
        }
    }
    None
}

fn find_digit_r2l(map: WordMap, line: &String) -> Option<u32> {
    let mut i = line.len();

    while i > 0 {
        let c = line.chars().nth(i - 1).unwrap();
        if c.is_numeric() {
            return c.to_digit(DECIMALS);
        }
        for (w, d) in map {
            if line[..i].ends_with(w) {
                return Some(d);
            }
        }
        i -= 1;
    }
    None
}

fn main() -> Result<(), IoError> {
    let word_map: WordMap = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let input_file_path =
        Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join(INPUT_FILE_PATH);

    let input = std::fs::read_to_string(input_file_path)?;

    let lines = input.lines();

    let mut sum = 0u32;

    for line in lines {
        let s = String::from(line);
        let l = find_digit_l2r(word_map, &s);
        let r = find_digit_r2l(word_map, &s);
        let val: u32 = format!("{}{}", l.unwrap(), r.unwrap()).parse().unwrap();
        sum += val;
        println!("{} -> {}", line, val);
    }

    println!("{}", sum);

    Ok(())
}
