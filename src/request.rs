use crate::utils::method::Method;

pub struct Request {
    #[cfg(not(feature = "url"))]
    pub url: String,

    #[cfg(feature = "url")]
    pub url: url::Url,

    pub method: Method,
    pub body: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    pub fn from(buffer: Vec<String>) -> Self {
        println!("{:#?}", buffer);
        let (method, url) = parse_request(&buffer[0]);
        
        Self {
            url,
            method,
            body: String::new(),
            headers: Vec::new(),
        }
    }
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