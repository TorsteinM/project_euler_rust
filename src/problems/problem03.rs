/// 
/// Problem 3: Hva er det største primtallet som er faktor til 600851475143?
/// 
/// Noe brute force aktig. Sikkert lurerer måter å gjøre dette på.
/// Men så lenge det er en del faktorer vi kan strippe av, så vil tallet fort bli mindre.


pub fn solve () -> u64 {
    let mut n:u64 = 600851475143;
    let mut i:u64 = 3;
    loop {
        if n%i>0 {      // Hvis n ikke er delelig på i
            i+=2;       // Prøv neste oddetall
            continue;   // Hopp til neste iterasjon
        }
        if n != i {     // Stripp av faktor hvis det ikke er det største
            n/=i;
            continue;
        }
        return n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem3() {
        assert_eq!(solve(), 6857);
    }
}