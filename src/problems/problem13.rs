/// 
/// Problem 13: Sum av store tall
/// 

use std::fs::File;
use std::io::{self, BufRead};

pub fn solve() -> u64 {

    let path = "src/problems/problem13.txt";

    let input = File::open(path).unwrap();
    let reader = io::BufReader::new(input);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // log2(10^50) = 50*log2(10) = 166.09640474436813   u128 holder ikke
    // log2(10^25) = 25*log2(10) = 83.04820237218406    u128 holder
    let mut lower_sum: u128 = 0;
    let mut upper_sum: u128 = 0;
    for line in lines {
        lower_sum += line[25..].parse::<u128>().unwrap();
        upper_sum += line[..25].parse::<u128>().unwrap();
    }
    upper_sum += lower_sum / 10u128.pow(25);
    
    (upper_sum / 10u128.pow(17)) as u64 // NB! 10^17 er hardkodet for akkurat disse tallene.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(), 5537376230);
    }
}