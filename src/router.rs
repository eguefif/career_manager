use crate::router::api_routes::route_api;
use crate::router::static_routes::route_static;
use webserv_rs::content_type::ContentType;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

pub mod api_routes;
pub mod static_routes;

#[derive(PartialEq, Debug)]
enum ResourceType {
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
        ResourceType::Static => route_static(request.uri.as_str()),
        ResourceType::Api => route_api(request),
        ResourceType::None => None,
    };
    if let Some(response) = retval {
        println!("Response: {} \n{:?}", response.status, response.headers);
        response
    } else {
        Response::new(400, vec![], vec![], ContentType::TextHtml)
    }
}

fn get_ressource_type_uri(uri: &str) -> ResourceType {
    let mut splits = uri.split("/");
    splits.next();
    if let Some(ressource) = splits.next() {
        if ressource == "api" {
            ResourceType::Api
        } else {
            ResourceType::Static
        }
    } else {
        ResourceType::None
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_get_api_ressource() {
        let uri = "/api/homepage/build";
        let result = get_ressource_type_uri(uri);

        assert_eq!(result, ResourceType::Api)
    }

    #[test]
    fn it_should_get_static_ressource() {
        let uri = "/images/yo.png";
        let result = get_ressource_type_uri(uri);

        assert_eq!(result, ResourceType::Static)
    }

    #[test]
    fn it_should_get_none_ressource() {
        let uri = "";
        let result = get_ressource_type_uri(uri);

        assert_eq!(result, ResourceType::None)
    }
}
