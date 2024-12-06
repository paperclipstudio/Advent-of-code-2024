use regex::Regex;
use std::env;
use std::fs;

pub fn part1() {
    let contents =
        fs::read_to_string("src/day3.txt").expect("Should have been able to read the file");
    let total = calc(&contents).unwrap();
}

pub fn part2() {
    let contents =
        fs::read_to_string("src/day3.txt").expect("Should have been able to read the file");
    let this = strip_dont(&contents);
    println!("{this}");
    let total = calc(&strip_dont(&contents)).unwrap();

    println!("{total}");
    // 98826679  <-- too high
    // 129087088 <-- too high
    // 174336360 <-- too high
}

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
    Some(this)
}

fn calc(string: &str) -> Option<i32> {
    Some(strip_mul(string)?.iter().map(|(a, b)| a * b).sum::<i32>())
}

fn strip_dont(string: &str) -> String {
    let re = Regex::new(r"don't\(\)(.|\n)*?(do\(\))").unwrap();
    re.replace_all(string, "").to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(vec![(3, 4)], strip_mul("mul(3,4)").unwrap())
    }

    #[test]
    fn simple_four_digit() {
        assert_eq!(Vec::<(i32, i32)>::new(), strip_mul("mul(3123,4)").unwrap());
        assert_eq!(Vec::<(i32, i32)>::new(), strip_mul("mul(3,1123)").unwrap());
        assert_eq!(Vec::<(i32, i32)>::new(), strip_mul("mul(3,)").unwrap());
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

    #[test]
    fn part2_simple() {
        assert_eq!("AAABBB", strip_dont("AAAdon't()ZXCVASDFdo()BBB"));
    }

    #[test]
    fn part2_many() {
        assert_eq!(
            "AAABBBCCC",
            strip_dont(
                "AAAdon't()ZXCVASDFdo()BBBdon't()ASDF#EIJFEdo(ddasdfjdo()CCCdon't()fdajcecakldjf"
            )
        );
    }

    #[test]
    fn part2_many_donts() {
        assert_eq!(
            "AAABBBCCC",
            strip_dont("AAAdon't()Fdon't()Gdon't()Gdo()BBBCCC")
        );
    }

    #[test]
    fn part2_many_dos() {
        assert_eq!(
            "AAAdo()Bdo()BBCCC",
            strip_dont("AAAdo()Bdo()BBdon't()Gdo()CCC")
        );
    }

    #[test]
    fn part2_all_dont() {
        assert_eq!("", strip_dont("don't()BBBdon't()GCCC"));
    }

    #[test]
    fn part2_all_dont2() {
        assert_eq!("", strip_dont("don't()BBBdon't()GCCCdo()"));
    }

    #[test]
    fn part2_dont_with_many_do() {
        assert_eq!(
            "AAABBBdo()CCCdo()",
            strip_dont("AAAdon't()zzzzdo()BBBdo()CCCdo()")
        );
    }

    #[test]
    fn part2_many_dont_with_many_do() {
        let a = strip_dont("AAAdon't()yyyydon't()zzzzdo()BBBdo()CCCdo()");
        println!("{a}");
        assert_eq!(
            "AAABBBdo()CCCdo()",
            strip_dont("AAAdon't()yyyydon't()zzzzdo()BBBdo()CCCdo()")
        );
    }

    const SAMPLE_STR: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn sample2() {
        let stripped = strip_dont(SAMPLE_STR);
        println!("{}", stripped);
        assert_eq!(48, calc(&stripped).unwrap());
    }
}
