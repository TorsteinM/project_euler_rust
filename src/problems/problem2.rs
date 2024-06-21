/// 
/// Problem 2: Summere partall i Fibonacci-tallserien under 4_000_000
/// 
/// Unroller løkken slik at vi lager 3 fibonaccitall per iterasjon og 
/// får det neste partallet i Fibonacci-tallserien uten å sjekke om det er partall.
/// 
///


pub fn solve() -> u64 {
    let mut sum: u64 = 0;
    let mut a: u64 = 1;
    let mut b: u64 = 2;
    while b < 4_000_000 {
        sum += b;           // sum: 2 i første iterasjon (første partall i Fibonacci-tallserien)
        let c = a + b; // c: 3 i første iterasjon
        a = b + c;          // a: 5 i første iterasjon
        b = c + a;          // b: 8 i første iterasjon (neste partall i Fibonacci-tallserien)
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem2() {
        assert_eq!(solve(), 4613732);
    }
}