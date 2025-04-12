use career_manager::{connector::SqlEngine, models::project::Project};
use serde_json;
use webserv_rs::{content_type::ContentType, response::Response};

pub fn index() -> Option<Response> {
    let mut engine = SqlEngine::new("cm.db");
    let projects = Project::all(&mut engine);
    if let Ok(projects) = serde_json::to_string(&projects) {
        return Some(Response::new(
            200,
            projects.as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ));
    }
    None
}
