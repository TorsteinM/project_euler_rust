/// 
/// Problem 45: Neste tall som er pentagonalt, trekanttall og heksagonalt
///
/// Løst med inversfunksjoner

pub fn solve() -> u64 {
    let mut n = 285;
    loop {
        n += 1;
        let triangle = n*(n + 1)/2;
        
        if is_pentagonal(triangle) && is_hexagonal(triangle) {
            return triangle;
        }
    }
}

fn is_pentagonal(n:u64) -> bool {
    (1.0 + f64::sqrt(((n*24) as f64)+ 1.0)) % 6.0 == 0.
}

fn is_hexagonal(n:u64) -> bool {
    (1.0 + f64::sqrt(((n*8) as f64) + 1.0)) % 4.0 == 0.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem45() {
        assert_eq!(solve(), 1533776805);
    }
}