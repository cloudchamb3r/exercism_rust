pub fn is_armstrong_number(num: u32) -> bool {
    let original: u64 = num.into();

    let sz = num.to_string().len().try_into().unwrap();
    let mut sum: u64 = 0;
    let mut num: u64 = num.into();
    while num != 0 {
        sum += (num % 10).pow(sz);
        if sum > original {
            return false;
        }
        num /= 10;
    }
    original == sum
}
