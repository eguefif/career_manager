pub mod connector;
pub mod models;
pub mod rendering;
pub mod website_builder;

pub fn log_error(message: &str) {
    eprintln!("\x1b[31mError: {}", message);
}
