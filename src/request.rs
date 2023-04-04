use std::net::TcpStream;

use crate::method::Method;

pub struct Request {
    pub path: String,
    pub method: Method,
    pub body: String,
}

impl Request {
    pub fn from(buffer: Vec<String>) -> Self {
        Self {
            path: String::new(),
            method: Method::POST,
            body: String::new(),
        }
    }
}