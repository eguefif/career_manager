use career_manager::{
    connector::SqlEngine, models::profile::Profile, website_builder::WebsiteBuilder,
};
use webserv_rs::{content_type::ContentType, response::Response};

pub fn build() -> Option<Response> {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    let body = if let Err(e) = cm.build() {
        eprintln!("Error: action building failed: {e}");
        "{\"result\": \"failed\"}".to_string()
    } else {
        "{\"result\": \"success\"}".to_string()
    };

    Some(Response::new(
        200,
        body.as_bytes().to_vec(),
        vec![],
        ContentType::Json,
    ))
}

pub fn profile() -> Option<Response> {
    let mut engine = SqlEngine::new("./cm.db");
    if let Some(profile) = Profile::take_first(&mut engine) {
        if let Ok(body) = serde_json::to_string(&profile) {
            return Some(Response::new(
                200,
                body.as_bytes().to_vec(),
                vec![],
                ContentType::Json,
            ));
        }
    }
    None
}

pub fn edit_profile(body: Vec<u8>) -> Option<Response> {
    let body = String::from_utf8_lossy(&body);
    println!("{}", body);
    if let Ok(updated_profile) = serde_json::from_str::<Profile>(&body) {
        let mut engine = SqlEngine::new("./cm.db");
        let mut profile = Profile::take_first(&mut engine)?;
        profile.update(updated_profile);
        let result = profile.save(&mut engine);
        return Some(Response::new(
            200,
            result.as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ));
    }
    None
}
