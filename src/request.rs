use crate::utils::method::Method;

pub struct Request {
    pub path: String,
    pub method: Method,
    pub body: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    pub fn from(buffer: Vec<String>) -> Self {
        Self {
            path: String::new(),
            method: Method::POST,
            body: String::new(),
            headers: Vec::new(),
        }
    }
}