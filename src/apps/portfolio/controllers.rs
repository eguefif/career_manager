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

pub fn delete_project(id: String) -> Option<Response> {
    None
}

pub fn add_project(body: Vec<u8>) -> Option<Response> {
    let (body, image) = get_image(&body);
    println!("In add project: {}", body);
    match serde_json::from_str::<Project>(&body) {
        Ok(mut project) => {
            println!("After");
            if project.picture.len() > 0 {
                write_image(image, &project.picture);
            }
            let mut engine = SqlEngine::new("cm.db");
            let result = project.save(&mut engine);
            return Some(Response::new(
                200,
                result.as_bytes().to_vec(),
                vec![],
                ContentType::Json,
            ));
        }
        Err(e) => eprintln!("Error serder: {e}"),
    }

    None
}

fn get_image(body: &[u8]) -> (String, &[u8]) {
    let iter = body.iter();
    let mut idx = 0;
    for (i, _) in iter.enumerate() {
        if i >= 2 {
            if String::from_utf8_lossy(&body[i - 2..=i]) == "###" {
                idx = i;
                break;
            }
        }
    }
    if idx + 1 == body.len() {
        (String::from_utf8_lossy(&body[..idx - 2]).to_string(), &[])
    } else {
        (
            String::from_utf8_lossy(&body[..idx - 2]).to_string(),
            &body[idx + 1..],
        )
    }
}

fn write_image(image: &[u8], filename: &str) {
    if image.len() > 0 {
        let file = format!("{}/images/{}", BASE_PATH, filename);
        let mut file = std::fs::File::create(file).unwrap();
        file.write_all(image).unwrap();
    }
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

    #[test]
    fn it_should_find_seperator_no_picture() {
        let payload = [97, 98, 99, 98, 97, 35, 35, 35];
        let (body, _) = get_image(&payload);
        assert_eq!(body.as_str(), "abcba");
    }
}
