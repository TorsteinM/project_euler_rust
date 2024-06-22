/// 
/// Problem 47: Distinkte primtallsfaktorer
/// 
/// Prøver variant uten datastruktur, men det tar tydeligvis mye tid likevel.
/// 


pub fn solve() -> u64 {
    let mut candidate = 600u64;
    const N_FACTORS: u64 = 4u64;
    loop {
        let mut passed = true;      // Flagg for å terminere indre løkken korrekt
        for i in (0..N_FACTORS).rev() {
            if !n_factors_equals(candidate + i, N_FACTORS){
                candidate += i + 1;
                passed = false;
                break;
            }
        }
        if passed {
            break;
        }
    }
    candidate
}

fn n_factors_equals (n:u64, p:u64) -> bool {
    let mut n = n;
    let mut i = 2;
    let mut factors = 0u64;
    while n > 1 {
        if n % i == 0 {
            factors += 1;
            while n % i == 0 {
                n /= i;
            }
        }
        i+=1;
    }

    factors == p
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(), 134_043);
    }
}
