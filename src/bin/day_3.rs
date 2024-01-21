use std::env;

use std::error::Error;
use std::io::Error as IoError;
use std::path::Path;

// const INPUT_FILE_PATH: &str = "src/bin/day_3_input.txt";
const INPUT_FILE_PATH: &str = "src/bin/day_3_part_1_calib.txt";
const PART_1_CALIB_ANSWER: u32 = 4361;

#[derive(Debug)]
struct Number {
    row: i32,
    col: i32,
    end: i32,
    num: i32,
}

impl Number {
    fn new(row: i32, col: i32, len: i32, num: i32) -> Number {
        Number {
            row,
            col,
            end: col + len,
            num,
        }
    }
}

#[derive(Debug)]
struct Symbol {
    row: i32,
    col: i32,
}

impl Symbol {
    fn new(row: i32, col: i32) -> Symbol {
        Symbol { row, col }
    }

    fn touches(&self, num: &Number) -> bool {
        // number ends >= sym col - 1
        if num.end < self.col - 1 {
            return false;
        }

        // number starts <= sym col + 1
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
        while let Some(chr) = chars.next() {
            if chr == '.' {
                continue;
            }
            if chr.is_numeric() {
                let mut num = String::from(chr);
                while let Some(nchr) = chars.next() {
                    if !nchr.is_numeric() {
                        break;
                    }
                    if nchr.is_numeric() {
                        num.push(nchr);
                    }
                }
                nums.push(Number::new(
                    row as i32,
                    col,
                    num.len() as i32,
                    num.parse().unwrap(),
                ))
            }
            syms.push(Symbol::new(row as i32, col));
            col += 1;
        }
    }
    (nums, syms)
}

fn run() -> Option<u32> {
    // let input_file_path =
    //     Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join(INPUT_FILE_PATH);

    // let input = std::fs::read_to_string(input_file_path).expect("failed to read file to string");

    // let matrix = input_to_matrix(input);

    // let num_pos: Vec<(u32, u32, u32)> = Vec::new();
    // let sym_pos: Vec<(u32, u32)> = Vec::new();

    // for row in matrix {
    //     for chr in row {
    //         if chr.is_numeric()
    //     }
    // }

    None
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_calib() {
        assert_eq!(run(), Some(PART_1_CALIB_ANSWER));
    }

    #[test]
    fn test_sym_touches() {
        let n = Number::new(4, 6, 3, 123);
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
            let s = Symbol::new(row, col);
            assert!(s.touches(&n));
        }
    }

    #[test]
    fn test_sym_not_touches() {
        let n = Number::new(4, 6, 3, 123);
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
            let s = Symbol::new(row, col);
            assert!(!s.touches(&n));
        }
    }
}
