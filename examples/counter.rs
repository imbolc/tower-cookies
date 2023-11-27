use axum::{routing::get, Router};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const COOKIE_NAME: &str = "visited";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
