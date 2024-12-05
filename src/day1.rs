use std::env;
use std::fs;

pub fn day1_1() {
    let contents = fs::read_to_string("src/day1.txt")
        .expect("Should have been able to read the file");
    let mut firsts = Vec::new();
    let mut seconds = Vec::new();
    for line in contents.lines() {
        let pair: Vec<_> = line.split("   ").collect();
        firsts.push(pair[0].parse::<i32>().unwrap());
        seconds.push(pair[1].parse::<i32>().unwrap());
    }
    firsts.sort();
    seconds.sort();
    let both : i32 = firsts.iter().zip(seconds.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{both:?}");
}

pub fn day1_2() {
    let contents = fs::read_to_string("src/day1.txt")
        .expect("Should have been able to read the file");
    let mut sum = 0;
    let mut firsts = Vec::new();
    let mut seconds = Vec::new();
    for line in contents.lines() {
        let pair: Vec<_> = line.split("   ").collect();
        firsts.push(pair[0].parse::<i32>().unwrap());
        seconds.push(pair[1].parse::<i32>().unwrap());
    }

    for id in firsts.iter() {
        sum += seconds.iter().filter(|x| *x == id).count() as i32 * id;
    }


    println!("{sum:?}");

    // A1 - B1 + A2 - B2 == A2 - B1 + A1 - B2
    // So we don't need to sort the lists

}
