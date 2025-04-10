use career_manager::website_builder::WebsiteBuilder;
use webserv_rs::{content_type::ContentType, response::Response};

pub fn build() -> Option<Response> {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    let body = if let Err(e) = cm.build() {
        eprintln!("Error: action building failed: {e}");
        "{\"result\": \"failed\"".to_string()
    } else {
        "{\"result\": \"success\"".to_string()
    };

    Some(Response::new(
        200,
        body.as_bytes().to_vec(),
        vec![],
        ContentType::Json,
    ))
}
