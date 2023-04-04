use std::{sync::Arc, net::TcpListener, io::{BufReader, BufRead}};

use crate::{route::{Route, RouteHandle}, utils::method::Method, request::Request, response::Response, threads::threadpool::ThreadPool};

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
        for stream in self.listener.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut stream) => {
                    let buffer: Vec<_> = BufReader::new(&mut stream)
                        .lines()
                        .map(|res| res.unwrap())
                        .take_while(|line| !line.is_empty())
                        .collect();

                    let route = self.get_route(&buffer[0].split(" ").collect::<Vec<&str>>()[1]);
                    if route.is_none() {
                        continue;
                    }

                    let handle = route.unwrap().get_handler().clone();

                    self.pool.execute(move || {
                        let request = Request::from(buffer);
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

    pub fn get_route(&self, path: &str) -> Option<&Route> {
        for route in &self.routes {
            if route.get_path() == path {
                return Some(route);
            }
        }

        None
    }

    pub fn get_route_with_method(&self, path: &str, method: Method) -> Option<&Route> {
        for route in &self.routes {
            if route.get_path() == path && route.get_method() == &method {
                return Some(route);
            }
        }
        return None
    }
}
