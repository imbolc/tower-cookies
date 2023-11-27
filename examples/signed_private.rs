//! The counter example using private / signed cookies instead of raw ones
//! Can be run by: `cargo run --all-features --example signed_private`
use axum::{routing::get, Router};
use once_cell::sync::OnceCell;
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies, Key};

const COOKIE_NAME: &str = "visited_private";
static KEY: OnceCell<Key> = OnceCell::new();

#[tokio::main]
async fn main() {
    let my_key: &[u8] = &[0; 64]; // Your real key must be cryptographically random
    KEY.set(Key::from(my_key)).ok();

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
    let key = KEY.get().unwrap();
    let private_cookies = cookies.private(key); // You can use `cookies.signed` as well

    let visited = private_cookies
        .get(COOKIE_NAME)
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    if visited > 10 {
        cookies.remove(Cookie::new(COOKIE_NAME, ""));
        "Counter has been reset".into()
    } else {
        private_cookies.add(Cookie::new(COOKIE_NAME, (visited + 1).to_string()));
        format!("You've been here {} times before", visited)
    }
}
