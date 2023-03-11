pub fn factors(n: u64) -> Vec<u64> {
    let mut ret = vec![];
    let mut n = n;
    for factor in 2.. {
        while n % factor == 0 {
            ret.push(factor);
            n /= factor;
        }
        if n == 1 {
            break
        }
    }
    ret
}
