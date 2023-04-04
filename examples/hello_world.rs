use std::sync::Arc;

use http_router::router::Router;

fn main() {
    let mut router = Router::create_server(3000);

    router.get("/", Arc::new(|req, res| {
        res.content("Hello World!");
        res.send_response();
    }));

    router.get("/blocking", Arc::new(|req, res| {
        std::thread::sleep(std::time::Duration::from_secs(5));
        res.content("Hello... (I have to wait 5 seconds!) ...World!");
        res.send_response();
    }));

    router.listen();
}