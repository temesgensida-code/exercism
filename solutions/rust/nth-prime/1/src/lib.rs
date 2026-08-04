pub fn list_primes(limit: u32) -> Vec<u32> {
    if limit < 2 {
        return vec![];
    }

    let limit_sz = limit as usize;
    let mut is_prime = vec![true; limit_sz + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let stop = (limit as f64).sqrt() as usize;
    for p in 2..=stop {
        if is_prime[p] {
            for multiple in (p * p..=limit_sz).step_by(p) {
                is_prime[multiple] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(num, &prime)| if prime { Some(num as u32) } else { None })
        .collect()
}

pub fn nth(n: u32) -> u32 {
    let primes = list_primes(200_000);
    // Assuming 0-based index. If 1-based, use (n - 1) as usize
    primes[n as usize]
}