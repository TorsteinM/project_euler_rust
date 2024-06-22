/// 
/// Problem 48: Eksponering og modularitet
///
/// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
/// 
/// Gjør først en brute force løsning
pub fn solve() -> u64 {
    let mut sum = 0u128;
    for i in 1..=1000u128 {
        let mut remainder = i;
        for _ in 1..i {
            remainder *= i;
            remainder %= 10u128.pow(10);
        }
        sum += remainder;
    }
    (sum % 10u128.pow(10)) as u64
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(), 9110846700);
    }
}