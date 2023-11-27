use axum::{routing::get, Router};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(cookies: Cookies) -> &'static str {
    cookies.add(Cookie::new("hello_world", "hello_world"));

    "Check your cookies."
}
