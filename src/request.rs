use std::{net::TcpStream, fmt::Debug};

use crate::utils::{method::Method, header::Header};

pub struct Request {
    #[cfg(not(feature = "url"))]
    pub url: String,

    #[cfg(feature = "url")]
    pub url: url::Url,

    pub method: Method,
    pub body: String,
    pub headers: Vec<Header>,
    pub ip: Option<String>,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Request {{")?;
        writeln!(f, "    url: {}, ", self.url)?;
        writeln!(f, "    method: {}, ", self.method)?;
        writeln!(f, "    body: {}, ", self.body)?;
        writeln!(f, "    headers: {:?}, ", self.headers)?;
        writeln!(f, "    ip: {:?}, ", self.ip)?;
        writeln!(f, "}}")
    }
}

impl Request {
    pub fn from(stream: &TcpStream, buffer: Vec<String>) -> Self {
        let (method, url) = parse_request(&buffer[0]);
        let ip = stream.peer_addr().ok().map(|addr| addr.ip().to_string());
        
        Self {
            url,
            method,
            body: parse_body(&buffer),
            headers: Vec::new(),
            ip,
        }
    }

    pub fn accepts(&self, content_type: &str) -> bool {
        self.headers.iter().any(|header| {
            header.key() == "Accept" && header.value().contains(content_type)
        })
    }
}

fn parse_body(buffer: &Vec<String>) -> String {
    let mut body = String::new();

    let binding = buffer.join("\r\n");
    let buffer = match binding.split("\r\n\r\n").nth(1) {
        Some(body) => body.lines(),
        None => return String::new(),
    };

    for line in buffer {
        body.push_str(line);
    }

    body
}

#[cfg(not(feature = "url"))]
fn parse_request(buffer: &str) -> (Method, String) {
    let mut line = buffer.lines().next().unwrap().split_whitespace();

    let method = line.next().unwrap();
    let url = line.next().unwrap();
    (Method::from_str(method), url.to_string())
}

#[cfg(feature = "url")]
fn parse_request(buffer: &str) -> (Method, url::Url) {
    let mut line = buffer.lines().next().unwrap().split_whitespace();

    let method = line.next().unwrap();
    let url = line.next().unwrap();
    (Method::from_str(method), url::Url::parse(url).unwrap())
}
