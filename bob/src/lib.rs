// OK, I really do not clearly understand requirements

pub fn reply(message: &str) -> &str {
    println!("!!!: {}", 'A'.is_uppercase());
    // How are you? -> Sure.
    // YELL -> Whoa, chill out!
    // YELL? -> Calm down, I know what I'm doing!
    // "" -> Fine. Be that way!
    // _ -> Whatever.
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    if 
        message.chars().all(|x|
            (x.is_ascii_alphabetic() && x.is_uppercase())
            || x.is_whitespace()
            || x == '\''
            || x == '?'
        )
        & (message.chars().rev().next() == Some('?'))
    {
        return "Calm down, I know what I'm doing!";
    }
    if message.chars().rev().next() == Some('?') {
        return "Sure.";
    }
    if
        message.chars().all(|x|
            (x.is_ascii_alphabetic() && x.is_uppercase())
            || x.is_digit(10)
            || x.is_whitespace()
            || x.is_ascii_punctuation()
            || x == '!'
        )
        & message.chars().any(|x| x.is_ascii_alphabetic())
    {
        return "Whoa, chill out!";
    }
    return "Whatever.";

}
