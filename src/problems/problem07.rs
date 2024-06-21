use crate::utils::is_prime;
/// 
/// Problem 7: Finn det 10001. primtallet
/// 
pub fn solve() -> u64 {
    let mut count = 2;
    let mut candidate = 3;
    while count < 10001 {
        candidate += 2;
        if is_prime(candidate) {
            count += 1;
        }
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem7() {
        assert_eq!(solve(), 104743);
    }
}