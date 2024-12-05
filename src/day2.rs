use std::env;
use std::fs;

pub fn part1() {
    let contents =
        fs::read_to_string("src/day2.txt").expect("Should have been able to read the file");
    let list_2d = contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solved2 = list_2d.iter().filter(|xs| is_safe(xs)).count();

    println!("{solved2:?}");
}

pub fn part2() {
    let contents =
        fs::read_to_string("src/day2.txt").expect("Should have been able to read the file");
    let list_2d = contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solved2 = list_2d.iter().filter(|xs| is_safe_buffer(xs)).count();

    println!("{solved2:?}");
}

fn is_safe_buffer(items: &[i32]) -> bool {
    if is_safe(items) {
        return true;
    }

    for item in 0..items.len() {
        let items_without_one: Vec<i32> = items[..item]
            .iter()
            .chain(items[item + 1..].iter())
            .inspect(|x| {
                print!("{x:?}, ");
            })
            .copied()
            .collect();
        println!();
        if is_safe(&items_without_one) {
            return true;
        }
    }

    false
}

fn is_safe(items: &[i32]) -> bool {
    (items.windows(2).all(|xs| xs[0] < xs[1]) || items.windows(2).all(|xs| xs[0] > xs[1]))
        && items.windows(2).all(|xs| (xs[0] - xs[1]).abs() >= 0)
        && items.windows(2).all(|xs| (xs[0] - xs[1]).abs() <= 3)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increasing() {
        assert!(is_safe(&[1, 2, 3, 4, 5]));
        assert!(!is_safe(&[5, 2, 3, 4, 5]));
        assert!(!is_safe(&[5, 2, 3, 4, 5]));
    }

    #[test]
    fn decreasing() {
        assert!(is_safe(&[5, 4, 3, 2, 1]));
        assert!(!is_safe(&[5, 2, 3, 4, 5]));
        assert!(!is_safe(&[5, 2, 3, 4, 5]));
    }

    #[test]
    fn at_least_1() {
        assert!(is_safe(&[5, 7, 10, 12, 13]));
        assert!(!is_safe(&[5, 5, 3, 2, 1]));
        assert!(!is_safe(&[5, 2, 3, 4, 5]));
        assert!(!is_safe(&[5, 2, 3, 4, 5]));
    }

    #[test]
    fn at_most_3() {
        assert!(is_safe(&[5, 7, 10, 12, 13]));
        assert!(!is_safe(&[3, 8, 9, 10, 11]));
        assert!(!is_safe(&[10, 6, 5, 4, 3, 2]));
    }

    #[test]
    fn part2_sample4() {
        assert!(is_safe_buffer(&[1, 3, 2, 4, 5]));
    }

    #[test]
    fn part2_sample5() {
        assert!(is_safe_buffer(&[8, 6, 4, 4, 1]));
    }

    #[test]
    fn part2_samples() {
        assert!(is_safe_buffer(&[7, 6, 4, 2, 1]));
        assert!(!is_safe_buffer(&[1, 2, 7, 8, 9]));
        assert!(!is_safe_buffer(&[9, 7, 6, 2, 1]));
        assert!(is_safe_buffer(&[1, 3, 6, 7, 9]));
    }
}
