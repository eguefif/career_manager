use career_manager::{connector::SqlEngine, log_error, models::article::Article};
use webserv_rs::{content_type::ContentType, response::Response};

pub fn index() -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    let articles = Article::all(&mut engine, None);
    if let Ok(articles) = serde_json::to_string(&articles) {
        println!("{:?}", articles);
        return Some(Response::new(
            200,
            articles.as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ));
    }
    None
}

pub fn new(body: Vec<u8>) -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    let body = String::from_utf8_lossy(&body);
    if let Ok(mut article) = serde_json::from_str::<Article>(&body) {
        article.save(&mut engine);
        return Some(Response::new(
            200,
            "{\"success\": true}".as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ));
    }
    None
}
