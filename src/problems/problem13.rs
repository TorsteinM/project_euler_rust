/// 
/// Problem 13: Sum av store tall
/// 

use std::{fs::File, io::BufRead};

pub fn solve() -> u64 {
    let file = File::open("src/problems/problem13.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut sum: u128 = 0;
    for line in reader.lines() {
        let number = line.unwrap().parse::<u128>().unwrap();
        sum += number;
    }
    sum.to_string()[0..10].parse::<u64>().unwrap()
}
