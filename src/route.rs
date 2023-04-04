use std::sync::Arc;

use crate::{request::Request, response::Response, method::Method};

pub type RouteHandle = dyn Fn(&Request, &mut Response) + Send + Sync + 'static;

pub struct Route {
    path: String,
    handler: Arc<RouteHandle>,
    method: Method,
}

impl Route {
    pub fn new(path: &str, method: Method, handler: Arc<RouteHandle>) -> Self {
        Self {
            path: path.to_string(),
            handler,
            method,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_handler(&self) -> &Arc<RouteHandle> {
        &self.handler
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }
}