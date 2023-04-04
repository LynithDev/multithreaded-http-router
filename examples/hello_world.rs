use std::{sync::Arc, thread::sleep, time::Duration};

use http_router::{router::Router};

fn main() {
    let mut router = Router::create_server(3000);

    router.get("/", Arc::new(|req, res| {
        res.send_file("./index.html");
        res.send();
    }));

    router.post("/post", Arc::new(|req, res| {
        res.content(&req.body);
        res.send();
    }));

    router.get("/blocking", Arc::new(|req, res| {
        sleep(Duration::from_secs(5));
        res.content("Hello... (I have to wait 5 seconds!) ...World!");
        res.send();
    }));

    router.listen();
}