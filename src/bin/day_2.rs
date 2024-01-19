use std::env;

use std::io::Error as IoError;
use std::path::Path;

// const INPUT_FILE_PATH: &str = "src/bin/day_2_input.txt";
const INPUT_FILE_PATH: &str = "src/bin/day_2_part_1_calib.txt";
const PART_1_CALIB_ANSWER: u32 = 8;

const DECIMALS: u32 = 10;

type ColorLimits = [(&'static str, u32); 3];

struct Try {
    red: u32,
    green: u32,
    blue: u32,
}

impl Try {}

struct Game {
    id: u32,
    tries: Vec<Try>,
}

impl Game {
    fn new(line: String) -> Game {
        let (game_str, tries_str) = line.split_once(':').unwrap();
        let game: u32 = game_str
            .replace("Game ", "")
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(DECIMALS)
            .unwrap();
        let tries: Vec<&str> = tries_str.split(';').map(|s| s.trim()).collect();
    }
}

fn main() -> Result<(), IoError> {
    let color_limits: ColorLimits = [("red", 12), ("green", 13), ("blue", 14)];

    let input_file_path =
        Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join(INPUT_FILE_PATH);

    let input = std::fs::read_to_string(input_file_path)?;

    let lines = input.lines();

    let mut sum = 0u32;

    for line in lines {}

    Ok(())
}
