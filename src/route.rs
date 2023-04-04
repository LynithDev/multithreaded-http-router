use std::{sync::Arc, fmt::{Display, Debug}};

use crate::{request::Request, response::Response, utils::method::Method};

pub type RouteHandle = dyn Fn(&Request, &mut Response) + Send + Sync + 'static;

#[derive(Clone)]
pub struct Route {
    path: String,
    handler: Arc<RouteHandle>,
    method: Method,
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Route {{")?;
        writeln!(f, "    path: {}, ", self.path)?;
        writeln!(f, "    method: {}, ", self.method)?;
        writeln!(f, "    handler: {:p}", self.handler)?;
        writeln!(f, "}}")
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route {{")?;
        write!(f, "    path: {}, ", self.path)?;
        write!(f, "    method: {}, ", self.method)?;
        write!(f, "    handler: {:p}", self.handler)?;
        write!(f, "}}")
    }
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