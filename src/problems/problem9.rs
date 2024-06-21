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