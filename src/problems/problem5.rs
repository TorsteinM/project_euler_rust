/// 
/// Problem 5: Finn det minste tallet som er delelig på alle tallene fra 1 til 20
/// 
/// Finner det minste antall faktorer for hvert primtall fra 1 til 20 og multipliserer dem sammen.
/// 
pub fn solve () -> u64 {
    let primes: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut minimum_exponents:[u64; 10] = [0; 10];

    for i in 2..21 {
        let mut n = i;
        let mut exponents:[u64; 10] = [0; 10];
        for j in 0..10 {
            while n % primes[j] == 0 {
                n /= primes[j];
                exponents[j] += 1;
            }
            if exponents[j] > minimum_exponents[j] {
                minimum_exponents[j] = exponents[j];
            }
        }
    }
    
    let mut result: u64 = 1;
    for i in 0..10 {
        result *= primes[i].pow(minimum_exponents[i] as u32);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem5() {
        assert_eq!(solve(), 232792560);
    }
}