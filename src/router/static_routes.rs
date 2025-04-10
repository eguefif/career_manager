use webserv_rs::content_type::ContentType;
use webserv_rs::response::Response;

const BASE_PATH: &str = "./html/admin/dev/";

pub fn route_static(uri: &str) -> Option<Response> {
    let static_ressource = match uri {
        "/" => Some(get_index()),
        "/portfolio" => Some(get_index()),
        "/blog" => Some(get_index()),
        "/bundle.js" => Some(get_bundle()),
        _ => get_asset(uri),
    };
    if let Some((body, content_type)) = static_ressource {
        Some(Response::new(200, body, vec![], content_type))
    } else {
        None
    }
}

fn get_index() -> (Vec<u8>, ContentType) {
    let index = std::fs::read_to_string(format!("{}/index.html", BASE_PATH)).unwrap();
    (index.as_bytes().to_vec(), ContentType::TextHtml)
}

fn get_bundle() -> (Vec<u8>, ContentType) {
    let index = std::fs::read_to_string(format!("{}/bundle.js", BASE_PATH)).unwrap();
    (index.as_bytes().to_vec(), ContentType::JS)
}

fn get_asset(uri: &str) -> Option<(Vec<u8>, ContentType)> {
    if uri.contains("css") {
        let css = std::fs::read_to_string(format!("{}/{}", BASE_PATH, uri)).unwrap();
        Some((css.as_bytes().to_vec(), ContentType::CSS))
    } else if uri.contains("favicon") {
        if let Ok(favicon) = std::fs::read(format!("{}/{}", BASE_PATH, uri)) {
            Some((favicon, ContentType::Icon))
        } else {
            None
        }
    } else if uri.contains("images") {
        if let Some(ext) = get_image_extension(uri) {
            if let Ok(image) = std::fs::read(format!("{}/{}", BASE_PATH, uri)) {
                return Some((image, ContentType::Image(ext.to_string())));
            }
        }
        None
    } else {
        None
    }
}

fn get_image_extension(uri: &str) -> Option<&str> {
    if let Some((_, extension)) = uri.rsplit_once(uri) {
        return Some(extension);
    }
    None
}
