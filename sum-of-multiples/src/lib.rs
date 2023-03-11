pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut s = std::collections::HashSet::new();

    for factor in factors.to_owned() { 
        if factor == 0 {
            s.insert(0);
            continue;
        }
        for i in (factor..limit).step_by(factor as usize) {
            s.insert(i);
        }
    }
    s.into_iter().sum()
}
