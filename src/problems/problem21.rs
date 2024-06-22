/// 
/// Problem 21: Amicable numbers
/// 
/// To tall som er sum av hverandres faktorer
/// 
use crate::utils::divisors;


pub fn solve() -> u64 {

    let mut sum = 0u64;
    
    for i in 2..10000 {
        let sum_divisors = divisors(i).iter().sum::<u64>() - i;
        if sum_divisors > i && divisors(sum_divisors).iter().sum::<u64>() - sum_divisors == i {
            sum += i + sum_divisors;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem21() {
        assert_eq!(solve(), 31626);
    }
}