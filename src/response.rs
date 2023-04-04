use std::{net::TcpStream, io::Write};

pub struct Response {
    status: u16,
    content: String,
    stream: TcpStream,
}

impl Response {
    pub fn from(stream: TcpStream) -> Self {
        Self {
            status: 200,
            content: String::new(),
            stream,
        }
    }

    pub fn status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }

    pub fn content(&mut self, body: &str) -> &mut Self {
        self.content = body.to_owned();
        self
    }

    pub fn send_response(&mut self) {
        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}", self.status, self.content.len(), self.content
        );
        
        match self.stream.write_all(response.as_bytes()) {
            Ok(_) => {},
            Err(e) => println!("{e}")
        };
    }

}