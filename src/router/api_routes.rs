use crate::apps::homepage;
use webserv_rs::{request::Request, response::Response};

pub fn route_api(request: Request) -> Option<Response> {
    let _body = String::from_utf8_lossy(&request.body);
    let action_route = get_action_route(&request.uri)?;
    match action_route {
        "homepage" => homepage::routes::route(request),
        _ => None,
    }
}

fn get_action_route(uri: &str) -> Option<&str> {
    let mut splits = uri.split("/");
    splits.next();
    splits.next();
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
        let uri = "/api/homepage";
        let result = get_action_route(uri).unwrap();

        assert_eq!(result, "homepage")
    }

    #[test]
    fn it_should_return_action_case_2() {
        let uri = "/api/homepage/";
        let result = get_action_route(uri).unwrap();

        assert_eq!(result, "homepage")
    }

    #[test]
    fn it_should_return_action_case_3() {
        let uri = "/api/homepage/build";
        let result = get_action_route(uri).unwrap();

        assert_eq!(result, "homepage")
    }
}
