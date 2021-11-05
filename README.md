[![License](https://img.shields.io/crates/l/tower-cookies.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/tower-cookies.svg)](https://crates.io/crates/tower-cookies)
[![Documentation](https://docs.rs/tower-cookies/badge.svg)](https://docs.rs/tower-cookies)

# tower-cookies

A cookie manager middleware built on top of [tower].

## Example

With [axum]:

```rust
use axum::{routing::get, Router};
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

A complete CRUD cookie example in [examples/counter.rs][example]

[axum]: https://crates.io/crates/axum
[tower]: https://crates.io/crates/tower
[example]: https://github.com/imbolc/tower-cookies/blob/main/examples/counter.rs

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Contributing

We appreciate all kinds of contributions, thank you!

### Note on README

The `README.md` file isn't meant to be changed directly. It instead generated from the crate's docs
by the [cargo-readme] command:

* Install the command if you don't have it: `cargo install cargo-readme`
* Change the crate-level docs in `src/lib.rs`, or wrapping text in `README.tpl`
* Apply the changes: `cargo readme > README.md`

If you have [rusty-hook] installed the changes will apply automatically on commit.

## License

This project is licensed under the [MIT license](LICENSE).

[cargo-readme]: https://github.com/livioribeiro/cargo-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
