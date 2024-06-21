use crate::utils::is_palindrome;
/// 
/// Problem 4: Finn det største palindromproduktet av to 3-sifrede tall
/// 
/// is_palindrome() bruker aritmetisk reversering

pub fn solve () -> u64 {
    let mut max: u64 = 0;

    for i in (100..1000).rev() {
        for j in (990..1000).rev() {
            let product = i * j;
            if is_palindrome(product) && product > max {
                max = product;
            }
        }
    }
    max
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem4() {
        assert_eq!(solve(), 906609);
    }
}