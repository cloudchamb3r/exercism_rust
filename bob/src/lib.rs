
fn is_question(msg: &str) -> bool {
    let msg = msg.trim();
    msg.ends_with('?')
}

fn is_yell(msg: &str) -> bool {
    let alphas = msg.chars().filter(|c| c.is_alphabetic());
    let uppercase_cnt = alphas.clone().fold(0, |acc, cur| {
        if cur.is_uppercase() {
            return acc + 1;
        }
        acc
    });
    let lowercase_cnt = alphas.clone().fold(0, |acc, cur| {
        if cur.is_lowercase() {
            return acc + 1;
        }
        acc
    });

    uppercase_cnt > 0 && lowercase_cnt == 0
}

fn is_silence(msg: &str) -> bool {
    msg.trim().is_empty()
}

pub fn reply(message: &str) -> &str {
    if is_yell(message) && is_question(message) {
        "Calm down, I know what I'm doing!"
    } else if is_question(message){
        "Sure."
    } else if is_yell(message) {
        "Whoa, chill out!"
    } else if is_silence(message) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
