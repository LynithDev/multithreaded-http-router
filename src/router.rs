use std::{sync::Arc, net::TcpListener, io::{BufReader, BufRead}};

use crate::{route::{Route, RouteHandle}, threadpool::ThreadPool, method::Method, request::Request, response::Response};

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

    pub fn get(&mut self, path: &str, handler: Arc<RouteHandle>) {
        self.routes.push(Route::new(path, Method::GET, handler));
    }

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
        self.routes.iter().find(|route| route.get_path() == path)
    }
}
