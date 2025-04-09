use token::Token;

use crate::rendering::render_error::RenderError;
use crate::rendering::token::get_token_type;
use crate::website_builder::Context;
use std::error::Error;
use std::io::Write;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
pub enum ValueType {
    Text(String),
}

mod render_error;
mod token;

const BUNDLE: &str = "./html/website/bundle.js";
const BASE_PATH: &str = "./html/website/templates/";
const BUNDLE_DIST: &str = "./html/dist/bundle.js";

pub fn render(context: Context) -> Result<(), Box<dyn Error>> {
    let content = render_template(BUNDLE, context)?;
    write_file(&content, BUNDLE_DIST)?;
    Ok(())
}

fn render_template(filename: &str, context: Context) -> Result<String, Box<dyn Error>> {
    let mut filled_template = String::new();
    if let Ok(template) = std::fs::read_to_string(&filename) {
        let mut iter = template.chars().peekable();
        while let Some(next) = iter.next() {
            if let Some(peek) = iter.peek() {
                if next == '{' && *peek == '{' {
                    iter.next();
                    let token = extract_token(&mut iter, &filename)?;
                    handle_token(&mut filled_template, &context, token)?;
                } else {
                    filled_template.push(next);
                }
            } else {
                break;
            }
        }
        return Ok(filled_template);
    } else {
        return Err(Box::new(RenderError::FileNotFound(filename.to_string())));
    }
}

fn handle_token(
    filled_template: &mut String,
    context: &Context,
    token: Token,
) -> Result<(), Box<dyn Error>> {
    match token {
        Token::Var(value) => {
            push_var(filled_template, &context, value);
        }
        Token::Use(value) => {
            push_template(filled_template, &context, value)?;
        }
    }
    Ok(())
}
fn extract_token<'a>(
    iter: &mut Peekable<Chars<'a>>,
    filename: &str,
) -> Result<Token, Box<dyn Error>> {
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
    Err(Box::new(RenderError::TokenError(filename.to_string())))
}
fn push_var(filled_template: &mut String, context: &Context, var: String) {
    let mut value: String = "".to_string();
    for (key, value_type) in context {
        if *key == var {
            value = match value_type {
                ValueType::Text(value) => value.to_string(),
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
    let nested_template = render_template(&filename, context.to_vec())?;
    filled_template.push_str(&nested_template.as_str());
    Ok(())
}

fn write_file(content: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
