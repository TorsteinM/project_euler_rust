/// 
/// Problem 1: Summere multipler av 3 og 5 under 1000
/// 

pub fn solve () -> u64  {
    let sum: u64 = (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    
    return sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(solve(), 233168);
    }
}