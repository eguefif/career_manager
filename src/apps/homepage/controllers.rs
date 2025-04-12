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
        Some(Response::new(
            200,
            profile.to_json(),
            vec![],
            ContentType::Json,
        ))
    } else {
        None
    }
}

pub fn edit_profile(body: Vec<u8>) -> Option<Response> {
    println!("{}", String::from_utf8_lossy(&body));
    // TODO: how do we update the profile. Several possibilities:
    // * do we retrieve value first
    // * when do we validate
    // * when do we save
    // * how do we deserialise
    // * do we have one function for all?
    None
}
