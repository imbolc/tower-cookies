[![version-badge][]][crate-url]
[![docs-badge][]][docs-url]
[![license-badge][]][crate-url]

# tower-cookies

## A [tower] ([axum]) cookies manager

### Usage

Here's an example of an axum app keeping track of your visits to the page (full example is in
[examples/counter.rs][example]):

```rust
#
let app = Router::new()
    .route(
        "/",
        get(|mut cookies: Cookies| async move {
            let visited = if let Some(cookie) = cookies.get("visited") {
                cookie.value().parse().ok().unwrap_or(0)
            } else {
                0
            };
            cookies.add(Cookie::new("visited", (visited + 1).to_string()));
            format!("You've been here {} times before", visited)
        }),
    )
    .layer(CookieLayer);
```

[tower]: https://crates.io/crates/tower
[axum]: https://crates.io/crates/axum
[example]: https://github.com/imbolc/tower-cookies/blob/main/examples/counter.rs

[version-badge]: https://img.shields.io/crates/v/tower-cookies.svg
[docs-badge]: https://docs.rs/tower-cookies/badge.svg
[license-badge]: https://img.shields.io/crates/l/tower-cookies.svg
[crate-url]: https://crates.io/crates/tower-cookies
[docs-url]: https://docs.rs/tower-cookies
