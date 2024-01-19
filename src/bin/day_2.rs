use std::env;

use std::io::Error as IoError;
use std::path::Path;

const INPUT_FILE_PATH: &str = "src/bin/day_2_input.txt";
// const INPUT_FILE_PATH: &str = "src/bin/day_2_part_1_calib.txt";
const PART_1_CALIB_ANSWER: u32 = 8;

const DECIMALS: u32 = 10;

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    pub fn new(line: String) -> Option<Game> {
        let (game_str, rounds_str) = Game::split_game_and_rounds(line)?;
        let id = Game::id_from_game_str(game_str)?;
        let (red, green, blue) = Game::max_vals_from_rounds_str(rounds_str)?;
        Some(Game {
            id,
            red,
            green,
            blue,
        })
    }

    fn split_game_and_rounds(line: String) -> Option<(String, String)> {
        line.split_once(':')
            .map(|v| (String::from(v.0), String::from(v.1)))
    }

    fn id_from_game_str(game: String) -> Option<u32> {
        Some(game.strip_prefix("Game ")?.parse::<u32>().unwrap_or(0))
    }

    fn max_vals_from_rounds_str(rounds: String) -> Option<(u32, u32, u32)> {
        let mut res: (u32, u32, u32) = (0, 0, 0);
        for round in rounds.split(';').map(|v| v.trim()) {
            for count in round.split(',').map(|v| v.trim()) {
                let color_val = count
                    .split_once(' ')
                    .map(|cv| (cv.0.parse::<u32>().unwrap_or(0), cv.1))?;
                match color_val {
                    (n, "red") => {
                        if n > res.0 {
                            res.0 = n
                        }
                    }
                    (n, "green") => {
                        if n > res.1 {
                            res.1 = n
                        }
                    }
                    (n, "blue") => {
                        if n > res.2 {
                            res.2 = n
                        }
                    }
                    _ => (),
                }
            }
        }
        Some(res)
    }

    fn is_possible(&self, limit: &Game) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() -> Result<(), IoError> {
    let limit = Box::new(Game {
        id: 0,
        red: 12,
        green: 13,
        blue: 14,
    });

    let input_file_path =
        Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join(INPUT_FILE_PATH);

    let input = std::fs::read_to_string(input_file_path)?;

    let lines = input.lines();

    let mut pos_sum = 0u32;
    let mut pow_sum = 0u32;

    for line in lines {
        let game = Game::new(String::from(line)).expect("can't get a game from line");
        if game.is_possible(limit.as_ref()) {
            pos_sum += game.id;
        }
        pow_sum += game.power();
    }

    println!("possible sum: {}", pos_sum);
    println!("power sum: {}", pow_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limit() -> Game {
        Game {
            id: 0,
            red: 12,
            green: 13,
            blue: 14,
        }
    }

    #[test]
    fn test_game_1() {
        let game = Game::new(String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        ))
        .unwrap();
        assert_eq!((game.id, game.red, game.green, game.blue), (1, 4, 2, 6));
        assert_eq!(game.power(), 48);
        assert!(game.is_possible(&limit()));
    }

    #[test]
    fn test_game_2() {
        let game = Game::new(String::from(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        ))
        .unwrap();
        assert_eq!((game.id, game.red, game.green, game.blue), (2, 1, 3, 4));
        assert_eq!(game.power(), 12);
        assert!(game.is_possible(&limit()))
    }

    #[test]
    fn test_game_3() {
        let game = Game::new(String::from(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        ))
        .unwrap();
        assert_eq!((game.id, game.red, game.green, game.blue), (3, 20, 13, 6));
        assert_eq!(game.power(), 1560);
        assert!(!game.is_possible(&limit()))
    }
}
