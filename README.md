[![version-badge][]][crate-url]
[![docs-badge][]][docs-url]
[![license-badge][]][crate-url]

# tower-cookies

## A [tower] ([axum]) cookies manager

### Usage

Here's an example of an axum app keeping track of your visits to the page (full example is in
[examples/counter.rs][example]):

```rust
let app = Router::new()
    .route(
        "/",
        // Using `Cookies` extractor to access cookies
        get(|mut cookies: Cookies| async move {
            let cookie_name = "visited";

            // Getting a cookie by it's name
            let visited = if let Some(cookie) = cookies.get(cookie_name) {
                cookie.value().parse().ok().unwrap_or(0)
            } else {
                0
            };

            if visited > 10 {
                // Removing the cookie
                cookies.remove(Cookie::new(cookie_name, ""));
                "counter has been reset".to_string()
            } else {
                // Adding (rewriting) the cookie
                cookies.add(Cookie::new(cookie_name, (visited + 1).to_string()));
                format!("You've been here {} times before", visited)
            }
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
