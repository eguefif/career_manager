use std::fmt;

#[derive(Debug)]
pub enum Token {
    Var(String),
    Use(String),
    For(String),
    End,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Var(value) => write!(f, "{{{{{}}}}}", value),
            Token::Use(value) => write!(f, "{{{{use(\"{}\")}}}}", value),
            Token::For(value) => write!(f, "{{{{for {}}}}}", value),
            Token::End => write!(f, "{{{{end}}}}"),
        }
    }
}

pub fn get_token_type(litteral: &str) -> Token {
    if litteral.contains("use") {
        let path = extract_use_path(litteral);
        Token::Use(path)
    } else if litteral.contains("for") {
        let path = extract_for_variable(litteral);
        Token::For(path)
    } else if litteral.trim() == "end" {
        Token::End
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

fn extract_for_variable(litteral: &str) -> String {
    let mut retval = String::new();
    let mut iter = litteral.chars();
    loop {
        if let Some(next) = iter.next() {
            if next == ' ' {
                while let Some(c) = iter.next() {
                    if c == '}' {
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
