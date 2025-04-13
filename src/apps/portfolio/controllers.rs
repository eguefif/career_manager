use crate::router::static_routes::BASE_PATH;
use career_manager::{connector::SqlEngine, models::project::Project};
use serde_json;
use std::io::Write;
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

pub fn add_project(body: Vec<u8>) -> Option<Response> {
    let (body, image) = get_image(&body);
    println!("In add project: {}", body);
    if let Ok(mut project) = serde_json::from_str::<Project>(&body) {
        write_image(image, &project.picture);
        let mut engine = SqlEngine::new("cm.db");
        let result = project.save(&mut engine);
        return Some(Response::new(
            200,
            result.as_bytes().to_vec(),
            vec![],
            ContentType::Json,
        ));
    }
    None
}

fn get_image(body: &[u8]) -> (String, &[u8]) {
    let iter = body.iter();
    let mut idx = 0;
    for (i, _) in iter.enumerate() {
        if i > 3 {
            println!("{}", String::from_utf8_lossy(&body[i - 3..i]));
            if String::from_utf8_lossy(&body[i - 3..i]) == "###" {
                idx = i;
                break;
            }
        }
    }
    println!("index: {}", idx);
    (
        String::from_utf8_lossy(&body[..idx - 3]).to_string(),
        &body[idx..],
    )
}

fn write_image(image: &[u8], filename: &str) {
    let file = format!("{}/images/{}", BASE_PATH, filename);
    let mut file = std::fs::File::create(file).unwrap();
    file.write_all(image).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_seperator() {
        let payload = [97, 98, 99, 98, 97, 35, 35, 35, 111, 65, 12];
        let expected_image = [111, 65, 12];
        let (body, image) = get_image(&payload);
        assert_eq!(body.as_str(), "abcba");
        for (i, value) in image.iter().enumerate() {
            assert_eq!(*value, expected_image[i]);
        }
    }
}
