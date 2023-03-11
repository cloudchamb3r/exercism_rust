pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret = vec![];
    if len > digits.len() {
        return ret;
    }
    
    for i in 0..=(digits.len() - len) { 
        ret.push(digits[i..(i + len)].to_owned());
    }
    ret
}
