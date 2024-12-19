use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

const SAMPLE_2: &str = "XXX
0123456789
MMMSXXMASM0
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

pub fn run() {
    let contents =
        fs::read_to_string("src/day5.txt").expect("Should have been able to read the file");
    let part1 = part_1(contents.clone());
    let part2 = part_2(contents);
    println!("part 1 -> {part1}");
    // Too low -> 217
    // Too low -> 6251
    println!("part 2 -> {part2}");
}

fn pages_are_safe(pages: &[i32], rules: &[(i32, i32)]) -> bool {
    for (index, page) in pages.iter().enumerate() {
        for other_page in pages.iter().skip(index + 1) {
            for (rule_1, rule_2) in rules {
                if rule_1 == other_page && rule_2 == page {
                    return false;
                }
            }
        }
    }
    true
}

fn page_is_safe_2(pages: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut cant_see = Vec::new();
    for page in pages {
        if cant_see.contains(page) {
            return false;
        }
        if !rules.contains_key(page) {
            continue;
        }
        cant_see.append(rules.get(page).unwrap().clone().as_mut());
    }
    true
}

fn middle_item(pages: &[i32]) -> &i32 {
    if pages.len() % 2 == 0 {
        panic!("Whats the middle page?");
    }
    pages.get((pages.len()) / 2).unwrap()
}

pub fn part_2(text: String) -> i32 {
    let rules_pairs = text
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|rules| rules.split('|').collect::<Vec<_>>())
        .map(|rules| {
            (
                rules.get(1).unwrap().parse::<i32>().unwrap(),
                rules.first().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for (when, cant) in rules_pairs {
        let cant_see = rules.entry(when).or_default();
        cant_see.push(cant);
    }

    // Rules are of the format After 'Key' you can't see 'value'

    let pages = text
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|pages| {
            pages
                .split(',')
                .map(|page| page.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|pages| !page_is_safe_2(pages, &rules))
        .map(|page| sort_pages(page.to_vec(), &rules))
        .inspect(|pages| println!("Sorted {pages:?}"))
        .map(|page| *middle_item(&page))
        .sum();
    pages
}

fn sort_pages(mut pages: Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut count = 0;
    while !page_is_safe_2(&pages, rules) {
        count += 1;
        if count > 1000 {
            return vec![42];
        }
        println!("{pages:?}");
        let mut cant_see = Vec::new();

        let mut swap = (0, 0);
        for (index, page) in pages.iter().enumerate() {
            if cant_see.contains(page) {
                swap = (index, index - 1);
                break;
            }
            if rules.contains_key(page) {
                cant_see.append(&mut rules.get(page).unwrap().clone())
            }
        }
        pages.swap(swap.0, swap.1);
    }
    pages
}

pub fn part_1(text: String) -> i32 {
    let rules = text
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|rules| rules.split('|').collect::<Vec<_>>())
        .map(|rules| {
            (
                rules.first().unwrap().parse::<i32>().unwrap(),
                rules.get(1).unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let pages = text
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|pages| {
            pages
                .split(',')
                .map(|page| page.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    pages
        .iter()
        .filter(|page| pages_are_safe(page, &rules))
        .map(|pages| middle_item(pages))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn none() {
        assert_eq!(143, part_1(String::from(SAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(123, part_2(String::from(SAMPLE)));
    }

    #[test]
    fn simple_rules() {
        let simpler_rules: HashMap<i32, Vec<i32>> =
            HashMap::from([(5, vec![4]), (4, vec![3]), (3, vec![2]), (2, vec![1])]);
        let simple_rules: HashMap<i32, Vec<i32>> = HashMap::from([
            (5, vec![1, 2, 3, 4]),
            (4, vec![1, 2, 3]),
            (3, vec![1, 2]),
            (2, vec![1]),
        ]);
    }

    #[test]
    fn simpler_rules() {
        let simpler_rules: HashMap<i32, Vec<i32>> =
            HashMap::from([(5, vec![4]), (4, vec![3]), (3, vec![2]), (2, vec![1])]);

        assert_eq!(
            vec!(1, 2, 3, 4, 5),
            sort_pages(vec![5, 4, 3, 2, 1], &simpler_rules)
        );
    }
}
