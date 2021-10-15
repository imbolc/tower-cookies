use axum::{handler::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieLayer, Cookies};

#[tokio::main]
async fn main() {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
