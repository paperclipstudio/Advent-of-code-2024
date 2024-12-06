use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

pub fn part1() {
    fn char_map(c: char) -> i8 {
        match c {
            'X' => 1,
            'M' => 2,
            'A' => 4,
            'S' => 8,
            _ => 0,
        }
    }

    let contents =
        fs::read_to_string("src/day4.txt").expect("Should have been able to read the file");
    let list_2d = contents
        .lines()
        .map(|x| x.chars().map(char_map).collect::<Vec<i8>>())
        .collect::<Vec<_>>();

    fn right(pos: (i32, i32)) -> (i32, i32) {
        (pos.0 + 1, pos.1)
    }

    let mut count = 0;
    /*
        for y in 0..list_2d.len() - 4 {
            for x in 0..list_2d[0].len() - 4 {
    */
    for y in 0..50 {
        for x in 0..50 {
            let right = (0..4).map(|x_1| list_2d[y][x_1 + x]);
            let down = (0..4).map(|y_1| list_2d[y_1 + y][x]);
            let right2 = right.clone().collect::<Vec<_>>();
            let diag = (0..3).map(|i| list_2d[y + i][x + i]);
            let windows = right
                .clone()
                .tuple_windows::<(i8, i8)>()
                .collect::<Vec<_>>();
            //            println!("{windows:?}");
            let forward = [1, 2, 4, 8];
            let reverse = [8, 4, 2, 1];
            if right2 == forward || right2 == reverse {
                count += 1;
                println!("{x} {y} {right2:?}");
            };
        }
    }
}
