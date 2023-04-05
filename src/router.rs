use std::{sync::Arc, net::TcpListener, io::{BufReader, Read}};

use crate::{route::{Route, RouteHandle}, utils::{method::Method, parser::get_header}, threads::threadpool::ThreadPool, request::Request, response::Response};

pub struct Router {
    routes: Vec<Route>,
    port: u16,
    pool: ThreadPool,
    listener: Option<TcpListener>
}

impl Router {
    pub fn create_server(port: u16) -> Self {
        Self {
            routes: Vec::new(),
            port,
            pool: ThreadPool::new(4),
            listener: None,
        }
    }

    pub fn create_server_with_pool(port: u16, pool: ThreadPool) -> Self {
        Self {
            routes: Vec::new(),
            port,
            pool,
            listener: None,
        }
    }

    // ALl methods from method.rs

    pub fn get(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::GET, handler));
    }

    pub fn post(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::POST, handler));
    }

    pub fn put(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::PUT, handler));
    }

    pub fn delete(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::DELETE, handler));
    }

    pub fn patch(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::PATCH, handler));
    }

    pub fn head(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::HEAD, handler));
    }

    pub fn options(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::OPTIONS, handler));
    }

    pub fn connect(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::CONNECT, handler));
    }

    pub fn trace(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::TRACE, handler));
    }

    pub fn any(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::ANY, handler));
    }

    // The server listeners / starteners

    pub fn listen(&mut self) {
        self.listen_handle(|| {});   
    }

    pub fn listen_handle<F>(&mut self, handle: F) where F: FnOnce() -> () {
        println!("Server started at http://localhost:{}/", self.port);
        
        self.listener = match TcpListener::bind(format!("0.0.0.0:{}", self.port)) {
            Ok(r) => {
                handle();
                Some(r)
            },
            Err(e) => return println!("{e}")
        };

        self.start_server();
    }

    fn start_server(&self) {

        let routes_arc = Arc::new(self.routes.clone());

        for stream in self.listener.as_ref().unwrap().incoming() {
            match stream {
                Ok(stream) => {
                    let routes = Arc::clone(&routes_arc);
                    self.pool.execute(move || {

                        let mut reader = BufReader::new(&stream);
                        let mut buffer = [0; 1024];
                        let mut message = Vec::<u8>::new();
                        let mut content_length = None;

                        loop {
                            let bytes_read = reader.read(&mut buffer).unwrap();

                            if bytes_read == 0 {
                                break;
                            }

                            if content_length.is_none() {
                                content_length = get_header(&buffer, "Content-Length").map(|s| s.parse::<usize>().unwrap());
                            }

                            message.extend_from_slice(&buffer[..bytes_read]);

                            if message.len() >= content_length.unwrap_or(0) {
                                break;
                            }
                        }

                        let message = String::from_utf8(message).unwrap().split("\n").map(|s| s.trim().to_owned()).collect::<Vec<_>>();

                        let route = routes.iter().find(|route| {
                            if message[0].split(" ").collect::<Vec<_>>().len() < 2 {
                                return false;
                            }

                            let path = message[0].split(" ").collect::<Vec<_>>()[1];

                            if route.get_method() == &Method::ANY {
                                return route.get_path() == path;
                            }

                            let method = message[0].split(" ").collect::<Vec<_>>()[0];

                            route.get_path() == path && route.get_method().to_str() == method
                        });

                        if route.is_none() {
                            return;
                        }

                        let handle = route.unwrap().get_handler().clone();

                        let request = Request::from(&stream, message);
                        let mut response = Response::from(stream);

                        handle(&request, &mut response);
                    });
                },
                Err(e) => println!("Error: {}", e)
            }
        }
    }

    // Getters

    pub fn get_routes(&self) -> &Vec<Route> {
        &self.routes
    }

    pub fn get_pool(&self) -> &ThreadPool {
        &self.pool
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    
}
