pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stk = vec![];

    for c in string.chars() {
        if c == '['  || c == '{' || c == '(' {
            if c == '[' {
                stk.push(']');
            } else if c == '{' {
                stk.push('}');
            } else if c == '(' {
                stk.push(')');
            }
        } else if c == ']'  || c == '}' || c == ')' {
            let last = stk.pop();
            if last.is_none() || last.unwrap() != c {
                return false;
            }
        }
    }
    stk.is_empty()
}
