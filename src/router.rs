use crate::router::api_ressource::route_api;
use crate::router::static_ressource::route_static;
use webserv_rs::content_type::ContentType;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

mod api_ressource;
mod static_ressource;

enum RessourceType {
    Static,
    Api,
}

pub fn route(request: Request) -> Response {
    let retval = match get_ressource_type_uri(&request.uri) {
        RessourceType::Static => route_static(request.uri.as_str()),
        RessourceType::Api => route_api(request),
    };
    if let Some((body, content_type)) = retval {
        Response::new(200, body.to_vec(), vec![], content_type)
    } else {
        Response::new(400, vec![], vec![], ContentType::TextHtml)
    }
}

fn get_ressource_type_uri(uri: &str) -> RessourceType {
    if uri.contains("/action") {
        return RessourceType::Api;
    }
    RessourceType::Static
}
