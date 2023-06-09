use super::status_code::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

pub struct Response {
    pub status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(i) => i,
            None => ""
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}