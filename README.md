[![License](https://img.shields.io/crates/l/tower-cookies.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/tower-cookies.svg)](https://crates.io/crates/tower-cookies)
[![Docs.rs](https://docs.rs/tower-cookies/badge.svg)](https://docs.rs/tower-cookies)

# tower-cookies

<!-- cargo-sync-readme start -->

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

<!-- cargo-sync-readme end -->


## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.


## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
