use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_cookies::{CookieManagerLayer, Cookies};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(cookies: Cookies) -> String {
    "foo".into()
    // let mut items = cookies
    //     .list()
    //     .iter()
    //     .map(|c| format!("{}={}", c.name(), c.value()))
    //     .collect::<Vec<_>>();
    // items.sort();
    // items.join(", ")
}
