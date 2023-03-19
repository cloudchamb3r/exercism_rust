pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();

    let mut last_space = true;
    let mut last_upper = false;
    for c in phrase.chars() {
        if c.is_uppercase()  && !last_upper{
            acronym.push(c);
            last_space = false;
            last_upper = true;
        } else if c.is_lowercase() && last_space {
            acronym.push(c.to_uppercase().next().unwrap());
            last_space = false;
            last_upper = false;
        } else {
            last_space = !c.is_alphabetic() && c != '\'';
            last_upper = c.is_uppercase();
        }
    }
    acronym
}

// HyperText sdklf sd