use crate::router::api_ressource::route_api;
use crate::router::static_ressource::route_static;
use webserv_rs::content_type::ContentType;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

mod api_ressource;
mod static_ressource;

#[derive(PartialEq, Debug)]
enum RessourceType {
    Static,
    Api,
    None,
}

pub fn route(request: Request) -> Response {
    let retval = match get_ressource_type_uri(&request.uri) {
        RessourceType::Static => route_static(request.uri.as_str()),
        RessourceType::Api => route_api(request),
        RessourceType::None => None,
    };
    if let Some((body, content_type)) = retval {
        Response::new(200, body.to_vec(), vec![], content_type)
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
