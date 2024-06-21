use crate::utils::divisors;

pub fn solve() -> u64 {
    let mut n = 1;
    let mut triangle = 0;
    loop {
        triangle += n;
        if divisors(triangle).len() > 500 {
            return triangle;
        }
        n += 1;
    }
}
