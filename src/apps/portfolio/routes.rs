use webserv_rs::{request::Request, response::Response};

use super::controllers::{add_project, delete_project, index, show_project, update_project};

pub fn route(request: Request) -> Option<Response> {
    let _body = String::from_utf8_lossy(&request.body);
    let action = get_controller_action(&request.uri)?;
    match action {
        "index" => index(),
        "new" => add_project(request.body),
        "delete" => delete_project(get_id(&request.uri)),
        "show" => show_project(get_id(&request.uri)),
        "update" => update_project(request.body, get_id(&request.uri)),
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

fn get_id(uri: &str) -> String {
    let (_, id) = uri.rsplit_once("/").unwrap();
    id.to_string()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_get_id() {
        let uri = "/api/portfolio/delete/11";
        let result = get_id(&uri);

        assert_eq!(result, "11")
    }
}
