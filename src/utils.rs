pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true

}

pub fn reverse_num(n: u64) -> u64 {
    let mut n:u64 = n;
    let mut i:u64 = 0;
    while n > 0{
        i *= 10;
        i += n%10;
        n /= 10;
    } 
    return i;
}

pub fn is_palindrome(n: u64) -> bool {
    n == reverse_num(n)
}

pub fn divisors(n: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    let mut i:u64 = 1;
    while i*i <= n {
        if n % i == 0 {
            divisors.push(i);
            if i != n/i {
                divisors.push(n/i);
            }
        }
        i += 1;
    }
    divisors
}

pub fn is_pentagonal(n: u64) -> bool {
    let x = (1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0;
    x == x.floor()
}
pub fn is_triangle(n: u64) -> bool {
    let x = (1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 2.0;
    x == x.floor()
}
pub fn is_hexagonal(n: u64) -> bool {
    let x = (1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 4.0;
    x == x.floor()
}