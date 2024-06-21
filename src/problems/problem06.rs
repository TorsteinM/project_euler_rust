/// 
/// Problem 6: Differanse mellom kvadrat av sum og sum av kvadrater
/// 
pub fn solve () -> u64 {
    let sum_of_squares: u64 = (1..101).map(|x| x*x).sum();
    let square_of_sum: u64 = (1..101).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem6() {
        assert_eq!(solve(), 25164150);
    }
}