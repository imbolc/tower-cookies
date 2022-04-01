//! The example illustrates accessing cookies from an
//! [`axum_core::extract::FromRequest::from_request`] implementation.
//! The behavior is the same as `examples/counter.rs` but cookies leveraging is moved into an
//! extractor.
use async_trait::async_trait;
use axum::{routing::get, Router};
use axum_core::extract::{FromRequest, RequestParts};
use http::StatusCode;
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const COOKIE_NAME: &str = "visited";

struct Counter(usize);

#[async_trait]
impl<B> FromRequest<B> for Counter
where
    B: Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookies = req.extensions().get::<Cookies>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract cookies. Is `CookieManagerLayer` enabled?",
        ))?;

        let visited = cookies
            .get(COOKIE_NAME)
            .and_then(|c| c.value().parse().ok())
            .unwrap_or(0)
            + 1;
        cookies.add(Cookie::new(COOKIE_NAME, visited.to_string()));

        Ok(Counter(visited))
    }
}

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

async fn handler(counter: Counter) -> String {
    format!("You have visited this page {} times", counter.0)
}
