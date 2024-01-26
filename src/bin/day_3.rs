use std::env;

use std::error::Error;
use std::fmt::{Display, Result};
use std::io::Error as IoError;
use std::path::Path;

// const INPUT_FILE_PATH: &str = "src/bin/day_3_input.txt";
const INPUT_FILE_PATH: &str = "src/bin/day_3_part_1_calib.txt";
const PART_1_CALIB_ANSWER: i32 = 4361;

#[derive(Debug)]
struct Number {
    row: i32,
    col: i32,
    end: i32,
    num: i32,
}

impl Number {
    fn new(row: i32, end: i32, num_str: String) -> Number {
        Number {
            row,
            col: end - num_str.len() as i32,
            end,
            num: num_str.parse().unwrap(),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Number<row: {}, col: {}, end: {}, num: {}",
            self.row, self.col, self.end, self.num
        )
    }
}

#[derive(Debug)]
struct Symbol {
    row: i32,
    col: i32,
    chr: String,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Symbol<row: {}, col: {}, chr: {}>",
            self.row, self.col, self.chr
        )
    }
}

impl Symbol {
    fn new(row: i32, col: i32, chr: String) -> Symbol {
        Symbol { row, col, chr }
    }

    fn is_adjacent(&self, num: &Number) -> bool {
        // number ends >= sym col - 1
        if num.end < self.col - 1 {
            return false;
        }

        // number starts <= sym + 1
        if num.col > self.col + 1 {
            return false;
        }

        // number row >= sym row - 1

        if num.row < self.row - 1 {
            return false;
        }

        // number row <= sym row + 1
        if num.row > self.row + 1 {
            return false;
        }

        true
    }
}

fn input_to_nums_and_syms(input: String) -> (Vec<Number>, Vec<Symbol>) {
    let mut nums: Vec<Number> = Vec::new();
    let mut syms: Vec<Symbol> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut chars = line.chars().into_iter().clone();
        let mut col = 0i32;
        let mut num_str = String::from("");
        let mut in_num = false;
        while let Some(chr) = chars.next() {
            if chr.is_numeric() {
                if !in_num {
                    in_num = true;
                }
                num_str.push(chr);
            } else {
                if in_num {
                    nums.push(Number::new(row as i32, col, num_str));
                    num_str = String::from("");
                    in_num = false;
                }
                if chr != '.' {
                    syms.push(Symbol::new(row as i32, col, chr.to_string()));
                }
            }
            col += 1;
        }
    }
    (nums, syms)
}

fn run() -> Option<i32> {
    let input_file_path =
        Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join(INPUT_FILE_PATH);

    let input = std::fs::read_to_string(input_file_path).expect("failed to read file to string");

    let (numbers, symbols) = input_to_nums_and_syms(input);

    let mut total = 0i32;
    for num in numbers.iter() {
        println!("{}", num);
        for sym in symbols.iter() {
            if sym.is_adjacent(num) {
                println!("     YES!    {}", sym);
                total += num.num;
                break;
            }
            println!("  *** NOPE *** {}", sym);
        }
    }

    Some(total)
}

fn main() {
    println!("total: {}", run().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "calib only"]
    fn test_part_1_calib() {
        assert_eq!(run(), Some(PART_1_CALIB_ANSWER));
    }

    #[test]
    fn test_sym_is_adjacent() {
        let n = Number::new(4, 9, "123".into());
        let coords: Vec<(i32, i32)> = vec![
            (3, 5),
            (3, 6),
            (3, 7),
            (3, 8),
            (3, 9),
            (3, 10),
            (4, 5),
            (4, 10),
            (5, 5),
            (5, 6),
            (5, 7),
            (5, 8),
            (5, 9),
            (5, 10),
        ];

        for (row, col) in coords {
            let s = Symbol::new(row, col, "!".into());
            assert!(s.is_adjacent(&n));
        }
    }

    #[test]
    fn test_sym_is_not_adjacent() {
        let n = Number::new(4, 9, "123".into());
        let coords: Vec<(i32, i32)> = vec![
            (2, 5),
            (2, 6),
            (2, 7),
            (2, 8),
            (2, 9),
            (2, 10),
            (4, 4),
            (4, 11),
            (6, 5),
            (6, 6),
            (6, 7),
            (6, 8),
            (6, 9),
            (6, 10),
        ];

        for (row, col) in coords {
            let s = Symbol::new(row, col, "!".into());
            assert!(!s.is_adjacent(&n));
        }
    }
}
