pub enum Token {
    Var(String),
    Use(String),
}

pub fn get_token_type(litteral: &str) -> Token {
    if litteral.contains("use") {
        let path = extract_use_path(litteral);
        Token::Use(path)
    } else {
        let place_holder = extract_place_holder(litteral);
        Token::Var(place_holder)
    }
}

fn extract_use_path(litteral: &str) -> String {
    let mut retval = String::new();
    let mut iter = litteral.chars();
    loop {
        if let Some(next) = iter.next() {
            if next == '(' {
                iter.next();
                while let Some(c) = iter.next() {
                    if c == '"' {
                        return retval;
                    }
                    retval.push(c);
                }
            }
        } else {
            break;
        }
    }
    retval
}

fn extract_place_holder(litteral: &str) -> String {
    let mut retval = String::new();
    for c in litteral.chars() {
        retval.push(c);
    }
    retval
}
