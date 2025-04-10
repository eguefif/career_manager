use crate::router::api_routes::route_api;
use crate::router::static_routes::route_static;
use webserv_rs::content_type::ContentType;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

mod api_routes;
mod static_routes;

#[derive(PartialEq, Debug)]
enum RessourceType {
    Static,
    Api,
    None,
}

/// There are two types of routing: Static and Api
/// # Static Routing
/// Static routing will return ressources for index.html, bundle.js and images.
///
/// # Api Routing
/// Api routing will route the request toward the right apps. Apis are divided in
/// apps in the folder Apps. Each apps has its own routing function.
/// To add a route api, you need to add an arm in the api_routes::route_api() function.
/// the action_Route has to match with the name of your app.
/// Then you need to add the route app function that will route the action to the right controllers
pub fn route(request: Request) -> Response {
    let retval = match get_ressource_type_uri(&request.uri) {
        RessourceType::Static => route_static(request.uri.as_str()),
        RessourceType::Api => route_api(request),
        RessourceType::None => None,
    };
    if let Some(response) = retval {
        response
    } else {
        Response::new(400, vec![], vec![], ContentType::TextHtml)
    }
}

fn get_ressource_type_uri(uri: &str) -> RessourceType {
    let mut splits = uri.split("/");
    splits.next();
    if let Some(ressource) = splits.next() {
        if ressource == "api" {
            RessourceType::Api
        } else {
            RessourceType::Static
        }
    } else {
        RessourceType::None
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_get_api_ressource() {
        let uri = "/api/homepage/build";
        let result = get_ressource_type_uri(uri);

        assert_eq!(result, RessourceType::Api)
    }

    #[test]
    fn it_should_get_static_ressource() {
        let uri = "/images/yo.png";
        let result = get_ressource_type_uri(uri);

        assert_eq!(result, RessourceType::Static)
    }

    #[test]
    fn it_should_get_none_ressource() {
        let uri = "";
        let result = get_ressource_type_uri(uri);

        assert_eq!(result, RessourceType::None)
    }
}
