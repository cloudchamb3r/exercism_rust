
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi  = array.len() as i32;

    while lo <= hi {
        let mid = ((lo + hi) / 2) as i32;
        if mid >= array.len() as i32{
            break;
        }
        match key.cmp(&array[mid as usize]) {
            std::cmp::Ordering::Less => hi = mid - 1,
            std::cmp::Ordering::Equal => return Some(mid as usize),
            std::cmp::Ordering::Greater => lo = mid + 1,
        }
    }
    None
}
