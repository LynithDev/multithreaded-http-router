use std::{net::TcpStream, io::Write, fs};

use crate::utils::status::StatusCode;

pub struct Response {
    status: StatusCode,
    content: String,
    headers: Vec<(String, String)>,
    stream: TcpStream,
}

impl Response {
    pub fn from(stream: TcpStream) -> Self {
        Self {
            status: StatusCode::Ok,
            content: String::new(),
            headers: Vec::new(),
            stream,
        }
    }

    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status;
        self
    }

    pub fn status_code(&mut self, status: u16) -> &mut Self {
        self.status = StatusCode::from_u16(status);
        self
    }

    pub fn content(&mut self, body: &str) -> &mut Self {
        self.content = body.to_owned();
        self
    }

    pub fn write(&mut self, body: &str) -> &mut Self {
        self.content.push_str(body);
        self
    }

    pub fn html(&mut self, body: &str) -> &mut Self {
        self.content = body.to_owned();
        self.add_header("Content-Type", "text/html");
        self
    }

    pub fn json(&mut self, body: &str) -> &mut Self {
        self.content = body.to_owned();
        self.add_header("Content-Type", "application/json");
        self
    }

    pub fn add_header(&mut self, header: &str, value: &str) -> &mut Self {
        self.headers.push((header.to_owned(), value.to_owned()));
        self
    }

    pub fn set_header(&mut self, header: &str, value: &str) -> &mut Self {
        self.headers = vec![(header.to_owned(), value.to_owned())];
        self
    }

    pub fn set_headers(&mut self, headers: Vec<(String, String)>) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn send_file(&mut self, path: &str) {
        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(_) => {
                self.status = StatusCode::NotFound;
                return;
            }
        };

        println!("{}", file);
        
        self.content(&file);
        self.status_code(200);
        self.add_header("Content-Type", "text/html");
        self.send();
    }

    pub fn send(&mut self) {
        let headers = if self.headers.len() > 0 { 
            self.headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\r\n") + "\r\n"
        } else { 
            String::new()
        };

        let builder = vec![
            format!("HTTP/1.1 {} {}", self.status.to_u16(), self.status.to_str()),
            format!("Content-Length: {}", self.content.len()),
            format!("Connection: close"),
            format!("{}", headers),
            format!("{}", self.content)
        ];
        
        match self.stream.write_all(builder.join("\r\n").as_bytes()) {
            Ok(_) => {},
            Err(e) => println!("{e}")
        };
    }

}