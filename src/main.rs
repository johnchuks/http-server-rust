mod server;
mod http;
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler);
}
