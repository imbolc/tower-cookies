[![License](https://img.shields.io/crates/l/tower-cookies.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/tower-cookies.svg)](https://crates.io/crates/tower-cookies)
[![Documentation](https://docs.rs/tower-cookies/badge.svg)](https://docs.rs/tower-cookies)

# tower-cookies

A cookie manager middleware built on top of [tower].

## Example

With [axum]:

```rust
use axum::{handler::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn handler(cookies: Cookies) -> &'static str {
    cookies.add(Cookie::new("hello_world", "hello_world"));

    "Check your cookies."
}
```
[axum]: https://crates.io/crates/axum
[tower]: https://crates.io/crates/tower

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## License

This project is licensed under the [MIT license](LICENSE).
