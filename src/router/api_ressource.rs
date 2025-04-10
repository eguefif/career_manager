use career_manager::website_builder::WebsiteBuilder;
use webserv_rs::{content_type::ContentType, request::Request};

pub fn route_api(request: Request) -> Option<(Vec<u8>, ContentType)> {
    let _body = String::from_utf8_lossy(&request.body);
    match request.uri.as_str() {
        "/action/build" => Some((build().as_bytes().to_vec(), ContentType::Json)),
        _ => None,
    }
}

fn build() -> String {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    if let Err(e) = cm.build() {
        eprintln!("Error: action building failed: {e}");
        return "{\"result\": \"failed\"".to_string();
    }
    return "{\"result\": \"success\"".to_string();
}
