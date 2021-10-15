use axum::{handler::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieLayer, Cookies};

#[tokio::main]
async fn main() {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
