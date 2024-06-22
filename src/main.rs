mod problems;
mod utils;
use std::time::Instant;

fn main() {


    let start = Instant::now();
    println!("Problem 1: {}, Tid: {} ns", problems::problem01::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 2: {}, Tid: {} ns", problems::problem02::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 3: {}, Tid: {} ns", problems::problem03::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 4: {}, Tid: {} ns", problems::problem04::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 5: {}, Tid: {} ns", problems::problem05::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 6: {}, Tid: {} ns", problems::problem06::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 7: {}, Tid: {} ns", problems::problem07::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 9: {}, Tid: {} ns", problems::problem09::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 10: {}, Tid: {} ns", problems::problem10::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 12: {}, Tid: {} ns", problems::problem12::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 13: {}, Tid: {} ns", problems::problem13::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 45: {}, Tid: {} ns", problems::problem45::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 47: {}, Tid: {} ns", problems::problem47::solve(), start.elapsed().as_nanos());
    let start = Instant::now();
    println!("Problem 48: {}, Tid: {} ns", problems::problem48::solve(), start.elapsed().as_nanos());
}
