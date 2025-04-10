use token::Token;

use crate::rendering::render_error::RenderError;
use crate::rendering::token::get_token_type;
use crate::website_builder::Context;
use std::error::Error;
use std::io::Write;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum ValueType {
    Text(String),
    List(Vec<ValueType>),
    Context(Box<Context>),
}

mod render_error;
mod token;

const BUNDLE: &str = "./html/website/bundle.js";
const BASE_PATH: &str = "./html/website/templates/";
const BUNDLE_DIST: &str = "./html/dist/bundle.js";

pub fn render(context: Context) -> Result<(), Box<dyn Error>> {
    let template = std::fs::read_to_string(BUNDLE)?;
    let content = render_template(&template, context)?;
    write_file(&content, BUNDLE_DIST)?;
    Ok(())
}

fn render_template(template: &str, context: Context) -> Result<String, Box<dyn Error>> {
    let mut filled_template = String::new();
    let mut iter = template.chars().peekable();
    while let Some(next) = iter.next() {
        if is_token(next, &mut iter) {
            let token = extract_token(&mut iter)?;
            handle_token(&mut filled_template, &context, token, &mut iter)?;
        } else {
            filled_template.push(next);
        }
    }
    return Ok(filled_template);
}

fn handle_token(
    filled_template: &mut String,
    context: &Context,
    token: Token,
    iter: &mut Peekable<Chars>,
) -> Result<(), Box<dyn Error>> {
    match token {
        Token::Var(value) => {
            push_var(filled_template, &context, value);
        }
        Token::Use(value) => {
            push_template(filled_template, &context, value)?;
        }
        Token::For(key) => {
            if let Some(items) = get_value(context, &key) {
                push_for(filled_template, items, iter)?;
            } else {
                return Err(Box::new(RenderError::MissingContextKey(key)));
            }
        }
        _ => {}
    }
    Ok(())
}

fn get_value<'a>(context: &'a Context, target_key: &str) -> Option<&'a ValueType> {
    for (key, value) in context {
        if key == target_key {
            return Some(value);
        }
    }
    None
}

fn extract_token<'a>(iter: &mut Peekable<Chars<'a>>) -> Result<Token, Box<dyn Error>> {
    let mut litteral = String::new();
    loop {
        if let Some(next) = iter.next() {
            if let Some(peek) = iter.peek() {
                if next == '}' && *peek == '}' {
                    iter.next();
                    return Ok(get_token_type(&litteral));
                } else {
                    litteral.push(next);
                }
            }
        } else {
            break;
        }
    }
    Err(Box::new(RenderError::TokenError))
}
fn push_var(filled_template: &mut String, context: &Context, var: String) {
    let mut value: String = "".to_string();
    for (key, value_type) in context {
        if *key == var {
            value = match value_type {
                ValueType::Text(value) => value.to_string(),
                _ => panic!("Should not push var for simething else than ValueType::Text"),
            };
            break;
        }
    }
    filled_template.push_str(&value);
}

fn push_template(
    filled_template: &mut String,
    context: &Context,
    var: String,
) -> Result<(), Box<dyn Error>> {
    let filename = format!("{}{}", BASE_PATH, var);
    if let Ok(template) = std::fs::read_to_string(&filename) {
        let nested_template = render_template(&template, context.to_vec())?;
        filled_template.push_str(&nested_template.as_str());
        Ok(())
    } else {
        Err(Box::new(RenderError::FileNotFound(filename)))
    }
}

fn push_for(
    filled_template: &mut String,
    items: &ValueType,
    iter: &mut Peekable<Chars>,
) -> Result<(), Box<dyn Error>> {
    let template = get_nested_template(iter)?;

    if let ValueType::List(contexts) = items {
        for context in contexts.iter() {
            if let ValueType::Context(context) = context {
                let template = render_template(&template, context.to_vec())?;
                filled_template.push_str(&template)
            } else {
                return Err(Box::new(RenderError::WrongValueTypeForForGen));
            }
        }
    } else {
        return Err(Box::new(RenderError::WrongValueTypeForForGen));
    }
    Ok(())
}

fn get_nested_template(iter: &mut Peekable<Chars>) -> Result<String, Box<dyn Error>> {
    let mut template = String::new();
    let mut scope_depth = 0;
    while let Some(next) = iter.next() {
        if is_token(next, iter) {
            let token = extract_token(iter)?;
            if is_nested_template_end(&token, &mut scope_depth) == true {
                return Ok(template);
            }
            template.push_str(&format!("{}", token));
        } else {
            template.push(next);
        }
    }
    Err(Box::new(RenderError::EOF))
}

fn is_token(next: char, iter: &mut Peekable<Chars>) -> bool {
    if let Some(peek) = iter.peek() {
        if next == '{' && *peek == '{' {
            iter.next();
            return true;
        }
    }
    false
}

fn is_nested_template_end(token: &Token, scope_depth: &mut usize) -> bool {
    match token {
        Token::For(_) => {
            *scope_depth += 1;
        }
        Token::End => {
            if *scope_depth == 0 {
                return true;
            } else {
                *scope_depth -= 1;
            }
        }
        _ => {}
    }
    false
}

fn write_file(content: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
