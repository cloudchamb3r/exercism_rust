
pub fn nth(n: u32) -> u32 {
    let max_val = 1_000_000;
    let mut is_sieve = vec![true; max_val];
    let mut primes = vec![];
    is_sieve[0] = false;
    is_sieve[1] = false;
    for i in 2..max_val {
        if is_sieve[i] != true {
            continue;
        }
        primes.push(i.try_into().unwrap());
        for j in ((2 * i)..max_val).step_by(i) {
            is_sieve[j] = false;
        }
    }
    primes[n as usize]
}
