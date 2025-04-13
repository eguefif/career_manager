use std::process::Command;

use career_manager::{
    connector::SqlEngine, log_error, models::profile::Profile, website_builder::WebsiteBuilder,
};
use webserv_rs::{content_type::ContentType, response::Response};

pub fn stop_preview() -> Option<Response> {
    let body = if let Ok(()) = run_stop_preview() {
        "{\"success\": true}".to_string()
    } else {
        "{\"success\": false}".to_string()
    };

    Some(Response::new(
        200,
        body.as_bytes().to_vec(),
        vec![],
        ContentType::Json,
    ))
}

fn run_stop_preview() -> Result<(), ()> {
    match Command::new("/bin/sh")
        .arg("./html/website/dist/stop_docker.sh")
        .status()
    {
        Ok(status) => {
            if status.success() {
                return Ok(());
            } else {
                Err(())
            }
        }
        Err(e) => {
            log_error(&format!("Error: Impossible to stop docker compose: {e}"));
            return Err(());
        }
    }
}

pub fn preview() -> Option<Response> {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    let body = if let Err(e) = cm.build() {
        log_error(&format!("Error: action building failed: {e}"));
        "{\"success\": false}".to_string()
    } else {
        if let Ok(()) = run_docker() {
            "{\"success\": true}".to_string()
        } else {
            "{\"success\": false}".to_string()
        }
    };

    Some(Response::new(
        200,
        body.as_bytes().to_vec(),
        vec![],
        ContentType::Json,
    ))
}

fn run_docker() -> Result<(), ()> {
    match Command::new("/bin/sh")
        .arg("./html/website/dist/run_docker.sh")
        .status()
    {
        Ok(status) => {
            if status.success() {
                return Ok(());
            } else {
                Err(())
            }
        }
        Err(e) => {
            log_error(&format!("Error: Impossible to run build: {e}"));
            return Err(());
        }
    }
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
