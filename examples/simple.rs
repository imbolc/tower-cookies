use axum::{handler::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieLayer, Cookies};

const COOKIE_NAME: &str = "axum-count";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(|mut cookies: Cookies| async move {
                let count = if let Some(cookie) = cookies.get(COOKIE_NAME) {
                    cookie.value().parse().ok().unwrap_or(0)
                } else {
                    0
                };
                cookies.add(Cookie::new(COOKIE_NAME, (count + 1).to_string()));
                format!("Count: {}", count)
            }),
        )
        .layer(CookieLayer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
