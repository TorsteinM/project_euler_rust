/// 
/// Problem 9: Divisible Triangle Number
/// 
/// Kanskje mulig å finne noen mønstre til å generere istedenfor å teste alle tallene.

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
