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

        let routes_arc = Arc::new(self.routes.clone());

        for stream in self.listener.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut stream) => {
                    let routes = Arc::clone(&routes_arc);
                    self.pool.execute(move || {
                        // TODO: Rewrite this to support body
                        // What you can do is set a limit of 1024 bytes, and if there is a Content-Length header, overwrite the limit with the value of the header
                        // Otherwise, just read 1024 bytes and then stop
                        
                        let buffer: Vec<_> = BufReader::new(&mut stream)
                            .lines()
                            .map(|res| res.unwrap())
                            .take_while(|line| !line.is_empty())
                            .collect();
                        
                        let route = routes.iter().find(|route| {
                            if buffer[0].split(" ").collect::<Vec<_>>().len() < 2 {
                                return false;
                            }

                            let path = buffer[0].split(" ").collect::<Vec<_>>()[1];

                            if route.get_method() == &Method::ANY {
                                return route.get_path() == path;
                            }

                            let method = buffer[0].split(" ").collect::<Vec<_>>()[0];

                            route.get_path() == path && route.get_method().to_str() == method
                        });

                        if route.is_none() {
                            return;
                        }

                        let handle = route.unwrap().get_handler().clone();


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

    
}
