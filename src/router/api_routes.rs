use crate::apps::{admin, article, portfolio, profile};
use webserv_rs::{request::Request, response::Response};

/// To add an api ressource, you need first to create an app in src/apps
/// You first add module file and a folder with the name of your app.
/// In the folder, you need two files:
/// * routes with a function route
/// * controllers with your controllers
///
/// There is an example with the homepage app.
///
/// Then you can add a route in this function.
pub fn route_api(request: Request) -> Option<Response> {
    let _body = String::from_utf8_lossy(&request.body);
    let action_route = get_action_route(&request.uri)?;
    match action_route {
        "admin" => admin::routes::route(request),
        "profile" => profile::routes::route(request),
        "portfolio" => portfolio::routes::route(request),
        "blog" => article::routes::route(request),
        _ => None,
    }
}

fn get_action_route(uri: &str) -> Option<&str> {
    let mut splits = uri.split("/");
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

    #[test]
    fn it_should_return_action_case_portfolio_index() {
        let uri = "/api/portfolio/index";
        let result = get_action_route(uri).unwrap();

        assert_eq!(result, "portfolio")
    }
}
