use std::{sync::Arc, thread::sleep, time::Duration};

use http_router::{router::Router, utils::status::StatusCode};

fn main() {
    let mut router = Router::create_server(3000);

    router.get("/", Arc::new(|req, res| {
        res.html("<h1>Hello World!</h1>");
        res.send();
    }));

    router.post("/", Arc::new(|req, res| {
        res.json(&req.body);
        res.send();
    }));

    router.get("/blocking", Arc::new(|req, res| {
        sleep(Duration::from_secs(5));
        res.status(StatusCode::Ok).content("I am multi-threaded!");
        res.send();
    }));

    router.listen();
}