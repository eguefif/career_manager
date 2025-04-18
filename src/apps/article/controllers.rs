use career_manager::{connector::SqlEngine, models::article::Article};
use webserv_rs::{content_type::ContentType, response::Response};

pub fn index() -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    let articles = Article::all(&mut engine, None);
    if let Ok(articles) = serde_json::to_string(&articles) {
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
        let result = article.save(&mut engine);
        return Some(Response::new(
            200,
            result.as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ));
    }
    None
}

pub fn delete(id: String) -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    if let Some(mut article) = Article::find(&mut engine, id.parse::<i64>().unwrap()) {
        let result = article.delete(&mut engine);

        Some(Response::new(
            200,
            result.as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ))
    } else {
        None
    }
}

pub fn update(id: String, body: Vec<u8>) -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    if let Some(mut article) = Article::find(&mut engine, id.parse::<i64>().unwrap()) {
        if let Ok(updates) = serde_json::from_str(&String::from_utf8_lossy(&body)) {
            let result = article.update(&mut engine, updates);

            return Some(Response::new(
                200,
                result.as_bytes().to_vec(),
                vec![],
                ContentType::Json,
            ));
        }
    }
    None
}

pub fn show(id: String) -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    if let Some(article) = Article::find(&mut engine, id.parse::<i64>().unwrap()) {
        if let Ok(article) = serde_json::to_string(&article) {
            return Some(Response::new(
                200,
                article.as_bytes().to_vec(),
                vec![],
                ContentType::Json,
            ));
        }
    }
    None
}
