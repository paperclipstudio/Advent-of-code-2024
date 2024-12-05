use regex::Regex;
use std::env;
use std::fs;

pub fn part1() {
    let contents =
        fs::read_to_string("src/day3.txt").expect("Should have been able to read the file");
    let total = calc(&contents).unwrap();
    println!("{total}");
}

pub fn part2() {}

fn strip_mul(string: &str) -> Option<Vec<(i32, i32)>> {
    //let re = Regex::new(r"mul(\\d{1,3},\\d{1,3})").unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let this = re
        .captures_iter(string)
        .map(|caps| {
            let (_, [first, second]) = caps.extract();
            (first.parse().unwrap(), second.parse().unwrap())
        })
        .collect();
    println!(">>>{this:?}");
    Some(this)
}

fn calc(string: &str) -> Option<i32> {
    Some(strip_mul(string)?.iter().map(|(a, b)| a * b).sum::<i32>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(vec![(3, 4)], strip_mul("mul(3,4)").unwrap())
    }

    #[test]
    fn strip() {
        assert_eq!(
            vec![(2, 4), (5, 5), (11, 8), (8, 5)],
            strip_mul("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
                .unwrap()
        );
    }

    #[test]
    fn sample() {
        assert_eq!(
            161,
            calc("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
                .unwrap()
        );
    }
}
