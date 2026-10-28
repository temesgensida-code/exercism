// pub fn factors(n: u64) -> Vec<u64> {
//     todo!("This should calculate the prime factors of {n}")
// }

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factor = Vec::new();

    // Handle the factor 2 first to allow skipping even numbers later
    while n % 2 == 0 {
        factor.push(2);
        n /= 2;
    }

    // Check odd numbers up to the square root of n
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factor.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        factor.push(n);
    }

    factor
}