/// 
/// Problem 10: Summering av primtall
///
/// Ikke særlig interessant løsning. 
/// Har prøvd sieve, men det blir ikke noe raskere løsning.
/// Mulig sieve var raskere på gammel maskinvare

use crate::utils::is_prime;

pub fn solve() -> u64 {
    (2..2_000_000).filter(|&x| is_prime(x)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem10() {
        assert_eq!(solve(), 142913828922);
    }
}