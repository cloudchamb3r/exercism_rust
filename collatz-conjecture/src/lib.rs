pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    let mut n = n;
    
    while true {
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            let mut of = false;
            if !of {
                of |= n.checked_mul(3).is_none();
            }
            if !of {
                of |= n.checked_mul(3).unwrap().checked_add(1).is_none();
            }
            if !of { 
                n = n.checked_mul(3).unwrap().checked_add(1).unwrap();
            }
            if of {
                return None;
            }
        }
        count += 1;
    }

    Some(count)
}
