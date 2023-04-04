# Multi-threaded HTTP Router in Rust
This is a simple HTTP router which is multi-threaded and can handle multiple requests at the same time.

> **Warning**
> This should not be used in production, as it is not secure and has no error handling. It is only meant for learning purposes.

## Usage
```rust
use std::{sync::Arc, thread::sleep, time::Duration};

use http_router::router::Router;

fn main() {
    let mut router = Router::create_server(3000);

    router.get("/", Arc::new(|req, res| {
        res.content("Hello World!");
        res.send_response();
    }));

    router.get("/blocking", Arc::new(|req, res| {
        sleep(Duration::from_secs(5));
        res.content("Hello... (I have to wait 5 seconds!) ...World!");
        res.send_response();
    }));

    router.listen();
}
```