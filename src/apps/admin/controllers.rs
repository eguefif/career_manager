use std::process::Command;

use career_manager::{
    log_error,
    website_builder::{copy_website_to_dist, WebsiteBuilder},
};

use webserv_rs::{content_type::ContentType, response::Response};

pub fn publish() -> Option<Response> {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    let body = if let Err(e) = cm.build() {
        log_error(&format!("Error: action building failed: {e}"));
        "{\"success\": false}".to_string()
    } else {
        if let Err(e) = copy_website_to_dist("./html/website/dist", "/home/eguefif/lab/website") {
            log_error(&format!("Error: copy to website failed: {e}"));
            return None;
        }
        push_git();
        "{\"success\": true}".to_string()
    };
    Some(Response::new(
        200,
        body.as_bytes().to_vec(),
        vec![],
        ContentType::Json,
    ))
}

fn push_git() {
    let _ = Command::new("sh").arg("run_git.sh").status();
}

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
