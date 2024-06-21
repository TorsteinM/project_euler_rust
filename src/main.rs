mod problems;
mod utils;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("Problem 1: {}, Tid: {} ns", problems::problem1::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 2: {}, Tid: {} ns", problems::problem2::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 3: {}, Tid: {} ns", problems::problem3::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 4: {}, Tid: {} ns", problems::problem4::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 5: {}, Tid: {} ns", problems::problem5::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 6: {}, Tid: {} ns", problems::problem6::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 7: {}, Tid: {} ns", problems::problem7::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 9: {}, Tid: {} ns", problems::problem9::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 10: {}, Tid: {} ns", problems::problem10::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 12: {}, Tid: {} ns", problems::problem12::solve(), start.elapsed().as_nanos());
}
