use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

const SAMPLE: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

pub fn part1() {
    let contents =
        fs::read_to_string("src/day4.txt").expect("Should have been able to read the file");
    let count = count_xmas(contents);
    println!("{count}");
}

fn count_xmas(contents: String) -> i32 {
    fn char_map(c: char) -> i8 {
        match c {
            'X' => 1,
            'M' => 2,
            'A' => 4,
            'S' => 8,
            _ => 0,
        }
    }
    let list_2d = contents
        .lines()
        .map(|x| x.chars().map(char_map).collect::<Vec<i8>>())
        .collect::<Vec<_>>();

    fn right(pos: (i32, i32)) -> (i32, i32) {
        (pos.0 + 1, pos.1)
    }

    let mut count = 0;
    for y in 0..list_2d.len() {
        for x in 0..list_2d[0].len() {
            let right = (0..4).map(|i| (i + x, y)).collect::<Vec<_>>();
            let down = (0..4).map(|i| (x, y + i)).collect::<Vec<_>>();
            let diag_right = (0..4).map(|i| (x + i, y + i)).collect::<Vec<_>>();
            let diag_left = (0..4).map(|i| (x - i.min(x), y + i)).collect::<Vec<_>>();
            let forward = [1, 2, 4, 8];
            let reverse = [8, 4, 2, 1];

            let all = [right, down, diag_right, diag_left].map(|pos| {
                pos.iter()
                    .map(|(x, y)| (*x, (*y).clamp(0, list_2d.len() - 1)))
                    .map(|(x, y)| (x.clamp(0, list_2d[y].len() - 20), y))
                    .map(|(x, y)| list_2d[y][x])
                    .collect::<Vec<_>>()
            });
            for set in all {
                println!("{set:?}");
                if set == forward || set == reverse {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
    // 326 too low
    // 2519 too low
    // 2538 too low
    // 2447 too low
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const NONE: &str = "OOOO
OOOO
OOOO
OOOO";

    const ONCE: &str = "XOOO
OMOO
OOAO
OOOS";

    const ONCE_R: &str = "OOOS
OOAO
OMOO
XOOO";
    const ONCE_U: &str = "SOOO
OAOO
OOMO
OOOX";
    #[test]
    fn none() {
        assert_eq!(0, count_xmas(String::from(NONE)));
    }

    #[test]
    fn once1() {
        assert_eq!(1, count_xmas(String::from(ONCE)));
    }

    #[test]
    fn once2() {
        assert_eq!(1, count_xmas(String::from(ONCE_R)));
    }

    #[test]
    fn once_u() {
        assert_eq!(1, count_xmas(String::from(ONCE_U)));
    }

    #[test]
    fn sample() {
        assert_eq!(18, count_xmas(String::from(SAMPLE)));
    }
}
