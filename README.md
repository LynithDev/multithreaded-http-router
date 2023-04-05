# Multi-threaded HTTP Router in Rust
This is a simple HTTP router which is multi-threaded and can handle multiple requests at the same time.

> **Warning**
> This should not be used in production, as it is not secure and has no error handling. It is only meant for learning purposes.

## Usage
```rust
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
```
To see more examples, check out the [examples folder](./examples)