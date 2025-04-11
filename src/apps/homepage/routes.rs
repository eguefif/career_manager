use webserv_rs::{request::Request, response::Response};

use super::controllers::{build, profile};

pub fn route(request: Request) -> Option<Response> {
    let _body = String::from_utf8_lossy(&request.body);
    let action = get_controller_action(&request.uri)?;
    match action {
        "build" => build(),
        "profile" => profile(),
        _ => None,
    }
}

fn get_controller_action(uri: &str) -> Option<&str> {
    let mut splits = uri.split("/");
    splits.next()?;
    splits.next()?;
    splits.next()?;
    if let Some(action) = splits.next() {
        if action.chars().last().unwrap() == '/' {
            let before_last = action.len() - 1;
            return Some(&action[..before_last]);
        }
        return Some(action);
    }
    None
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_return_action_case_1() {
        let uri = "/api/homepage/build";
        let result = get_controller_action(uri).unwrap();

        assert_eq!(result, "build")
    }
}
