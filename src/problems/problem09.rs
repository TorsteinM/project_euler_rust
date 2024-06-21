/// 
/// Problem 9: Special Pythagorean triplet
///
/// Ikke noe spesielt her. Begynner å telle fra toppen med b,
/// og holder a + b + c = 1000 ved å justere a og c for hver iterasjon.

pub fn solve() -> u64 {
    for b in (1..500 as u64).rev() {
        let mut a:u64 = 1;
        let mut c:u64 = 1000 - b - a;
        while a < b && b < c {
            if a*a + b*b == c*c {
                return a*b*c as u64;
            }
            a += 1;
            c -= 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem9() {
        assert_eq!(solve(), 31875000);
    }
}