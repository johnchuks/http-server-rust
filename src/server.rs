use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request, Response, StatusCode, request::ParseError};


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("An error occurred: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}



pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Server{addr}
    }

    pub fn run(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        // accept connections and process them serially
        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    println!("Failed to parse buffer: {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to Send Response: {}", e)
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                },
                Err(e) => println!("An error occurred establishing a connection: {}", e)
            }
        }
    }
}
