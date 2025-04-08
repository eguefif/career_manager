use crate::router::route;
use webserv_rs::http_server::HttpServer;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

pub mod router;

fn handle_client(request: Request) -> Response {
    route(request)
}

fn main() -> std::io::Result<()> {
    //let config =
    //    std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut server = HttpServer::new("127.0.0.1", 8080)?;
    server.run(handle_client)?;

    Ok(())
}
