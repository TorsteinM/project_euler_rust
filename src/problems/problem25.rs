/// 
/// Problem 25: Fibonacci tall med 1000 siffer
/// 
/// 


pub fn solve() -> u64 {
    let sqrt_5:f64 = f64::sqrt(5.0);
    let phi:f64 = (1.0 + sqrt_5)/2.0;
    
    // Invers og logaritmer
    let n = (999. + sqrt_5.log10())/phi.log10();

    n.ceil() as u64
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem25() {
        assert_eq!(solve(), 4782);
    }
}