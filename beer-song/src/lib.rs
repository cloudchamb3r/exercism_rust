pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1{
        format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    } else if n == 2 {
        format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n - 1)
    } else {
        format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n - 1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().into_iter()
    .fold(String::new(),|acc, cur| {
        if cur == end {
            return format!("{acc}{}", verse(cur));
        }
        format!("{acc}{}\n", verse(cur))
    })
}
