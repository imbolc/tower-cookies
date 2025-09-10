[![License](https://img.shields.io/crates/l/tower-cookies.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/tower-cookies.svg)](https://crates.io/crates/tower-cookies)
[![Docs.rs](https://docs.rs/tower-cookies/badge.svg)](https://docs.rs/tower-cookies)

# tower-cookies

A cookie manager middleware built on top of [tower].

## Example

With [axum]:

```rust,no_run
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handler(cookies: Cookies) -> &'static str {
    cookies.add(Cookie::new("hello_world", "hello_world"));

    "Check your cookies."
}
```

A complete CRUD cookie example in [examples/counter.rs][example]

[axum]: https://crates.io/crates/axum
[tower]: https://crates.io/crates/tower
[example]: https://github.com/imbolc/tower-cookies/blob/main/examples/counter.rs

## Contributing

Please run [.pre-commit.sh] before sending a PR, it will check everything.

## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]:
  https://github.com/imbolc/tower-cookies/blob/main/.pre-commit.sh
[license]: https://github.com/imbolc/tower-cookies/blob/main/LICENSE
