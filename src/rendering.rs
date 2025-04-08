use crate::website_builder::Context;

#[derive(Clone)]
pub enum ValueType {
    Text(String),
}

const BASE_PATH: &str = "./html/website";

pub fn render(template: &str, context: Context) -> String {
    "".to_string()
}
