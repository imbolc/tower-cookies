use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const COOKIE_NAME: &str = "visited";

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

async fn handler(cookies: Cookies) -> String {
    let visited = cookies
        .get(COOKIE_NAME)
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    if visited > 10 {
        cookies.remove(Cookie::new(COOKIE_NAME, ""));
        "Counter has been reset".into()
    } else {
        cookies.add(Cookie::new(COOKIE_NAME, (visited + 1).to_string()));
        format!("You've been here {} times before", visited)
    }
}
