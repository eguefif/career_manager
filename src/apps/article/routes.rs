use webserv_rs::{request::Request, response::Response};

use crate::apps::portfolio::routes::get_id;

use super::controllers::{delete, edit, index, new};

pub fn route(request: Request) -> Option<Response> {
    let _body = String::from_utf8_lossy(&request.body);
    let action = get_controller_action(&request.uri)?;
    match action {
        "index" => index(),
        "new" => new(request.body),
        "delete" => delete(get_id(&request.uri)),
        "edit" => edit(get_id(&request.uri), request.body),
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
