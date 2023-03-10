pub fn build_proverb(list: &[&str]) -> String {
    let mut ret = String::with_capacity(1000);

    for i in 1..list.len() {
        ret.push_str(format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]).as_str());
    }
    if !list.is_empty() {
        ret.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    }    
    ret
}
