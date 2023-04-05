
use super::server::Handler;
use super::http::{Request, StatusCode, Method, Response};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Works</h1>".to_string())),
                "/test" => Response::new(StatusCode::Ok, Some("<h1>Here is a test page</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None)
            }
            _  => Response::new(StatusCode::NotFound, None)
        }
    }
}