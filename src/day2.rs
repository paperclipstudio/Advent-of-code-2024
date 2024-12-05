use std::env;
use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("src/day2.txt")
        .expect("Should have been able to read the file");
    let list_2d = contents.lines()
        .map(|line| {
            line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let increasing = list_2d.iter().map(|line| {
        line.windows(2).map(|xs| xs[0] < xs[1]).all(|x| x)
    }).collect::<Vec<_>>();

    let decreasing = list_2d.iter().map(|line| {
        line.windows(2).map(|xs| xs[0] > xs[1]).all(|x| x)
    }).collect::<Vec<_>>();

    let at_least_1 = list_2d.iter().map(|line| {
        line.windows(2).map(|xs| (xs[0] - xs[1]).abs() > 0).all(|x| x)
    }).collect::<Vec<_>>();

    let at_most_3 = list_2d.iter().map(|line| {
        line.windows(2).map(|xs| (xs[0] - xs[1]).abs() < 4).all(|x| x)
    }).collect::<Vec<_>>();

        println!("{decreasing:?}");
        println!("{increasing:?}");
        println!("{at_least_1:?}");
        println!("{at_most_3:?}");


    let solved = increasing.iter().zip(decreasing).map(|(a,b)| a | b)
        .zip(at_least_1).map(|(a,b)| a & b)
        .zip(at_most_3).map(|(a,b)| a & b).filter(|x| *x).count();

    println!("{solved:?}");



}

pub fn _day1_2() {
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
