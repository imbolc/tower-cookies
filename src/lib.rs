//! # A [tower] ([axum]) cookies manager
//!
//! ## Usage
//!
//! Here's an example of an axum app keeping track of your visits to the page (full example is in
//! [examples/counter.rs][example]):
//!
//!```rust,no_run
//! # use axum::{handler::get, Router};
//! # use std::net::SocketAddr;
//! # use tower_cookies::{Cookie, CookieLayer, Cookies};
//! #
//! # #[tokio::main]
//! # async fn main() {
//! let app = Router::new()
//!     .route(
//!         "/",
//!         get(|mut cookies: Cookies| async move {
//!             let visited = if let Some(cookie) = cookies.get("visited") {
//!                 cookie.value().parse().ok().unwrap_or(0)
//!             } else {
//!                 0
//!             };
//!             cookies.add(Cookie::new("visited", (visited + 1).to_string()));
//!             format!("You've been here {} times before", visited)
//!         }),
//!     )
//!     .layer(CookieLayer);
//! #     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//! #     axum::Server::bind(&addr)
//! #         .serve(app.into_make_service())
//! #         .await
//! #         .unwrap();
//! # }
//! ```
//!
//! [tower]: https://crates.io/crates/tower
//! [axum]: https://crates.io/crates/axum
//! [example]: https://github.com/imbolc/tower-cookies/blob/main/examples/counter.rs

pub use cookie::Cookie;
use cookie::CookieJar;
use futures_util::ready;
use http::{header, HeaderValue, Request, Response};
#[cfg(feature = "tower-layer")]
pub use layer::CookieLayer;
use parking_lot::Mutex;
use pin_project_lite::pin_project;
use std::future::Future;
use std::sync::Arc;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

#[cfg(feature = "tower-layer")]
pub mod layer;

#[cfg(feature = "axum")]
pub mod extract;

/// A parsed on-demand cookie jar, can be used as an axum extractor
#[derive(Clone, Debug)]
pub struct Cookies {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Default)]
struct Inner {
    header: Option<HeaderValue>,
    jar: Option<CookieJar>,
    changed: bool,
}

impl Cookies {
    fn new(header: Option<HeaderValue>) -> Self {
        let inner = Inner {
            header,
            ..Default::default()
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    /// Adds cookie to this jar. If a cookie with the same name already exists, it is replaced with
    /// cookie.
    pub fn add(&mut self, cookie: Cookie<'static>) {
        let mut inner = self.inner.lock();
        inner.changed = true;
        inner.jar().add(cookie);
    }

    /// Returns the Cookie with the given name. If no such cookie exists, returns None.
    pub fn get(&mut self, name: &str) -> Option<Cookie> {
        let mut inner = self.inner.lock();
        inner.changed = true;
        inner.jar().get(name).cloned()
    }

    /// Removes cookie from this jar.
    pub fn remove(&mut self, cookie: Cookie<'static>) {
        let mut inner = self.inner.lock();
        inner.changed = true;
        inner.jar().remove(cookie);
    }

    /// Returns all the cookies present in this jar. It collects cookies into a vector instead of
    /// iterating through them to minimize the mutex locking time.
    pub fn list(&mut self) -> Vec<Cookie> {
        let mut inner = self.inner.lock();
        inner.jar().iter().cloned().collect()
    }
}

impl Inner {
    /// Cached jar
    fn jar(&mut self) -> &mut CookieJar {
        if self.jar.is_none() {
            let jar = self
                .header
                .as_ref()
                .and_then(|h| std::str::from_utf8(h.as_bytes()).ok())
                .map(|s| jar_from_str(s))
                .unwrap_or_default();
            self.jar = Some(jar);
        }
        self.jar.as_mut().unwrap()
    }
}

fn jar_from_str(s: &str) -> CookieJar {
    let mut jar = CookieJar::new();
    for cookie_str in s.split(';').map(str::trim) {
        if let Ok(cookie) = cookie::Cookie::parse_encoded(cookie_str) {
            jar.add_original(cookie.into_owned());
        }
    }
    jar
}

/// A tower service to put `Cookies` into `Request.extensions` and then apply possible changes
/// to the response.
#[derive(Clone, Debug)]
pub struct CookieService<S> {
    inner: S,
}

impl<S> CookieService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<ReqBody, ResBody, S> Service<Request<ReqBody>> for CookieService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let value = req.headers().get(header::COOKIE).cloned();
        let cookies = Cookies::new(value);
        req.extensions_mut().insert(cookies.clone());

        ResponseFuture {
            future: self.inner.call(req),
            cookies,
        }
    }
}

pin_project! {
    /// Response future for [`CookieService`].
    #[derive(Debug)]
    pub struct ResponseFuture<F> {
        #[pin]
        future: F,
        cookies: Cookies,
    }
}

impl<F, ResBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut res = ready!(this.future.poll(cx)?);

        let mut cookies = this.cookies.inner.lock();
        if cookies.changed {
            let values: Vec<_> = cookies
                .jar()
                .delta()
                .filter_map(|c| HeaderValue::from_str(&c.to_string()).ok())
                .collect();
            let headers = res.headers_mut();
            for value in values {
                headers.append(header::SET_COOKIE, value);
            }
        }

        Poll::Ready(Ok(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, BoxBody},
        handler::get,
        routing::BoxRoute,
        Router,
    };
    use tower::ServiceExt;

    fn app() -> Router<BoxRoute> {
        Router::new()
            .route(
                "/list",
                get(|mut cookies: Cookies| async move {
                    let mut items = cookies
                        .list()
                        .iter()
                        .map(|c| format!("{}={}", c.name(), c.value()))
                        .collect::<Vec<_>>();
                    items.sort();
                    items.join(", ")
                }),
            )
            .route(
                "/add",
                get(|mut cookies: Cookies| async move {
                    cookies.add(Cookie::new("baz", "3"));
                    cookies.add(Cookie::new("spam", "4"));
                }),
            )
            .route(
                "/remove",
                get(|mut cookies: Cookies| async move {
                    cookies.remove(Cookie::new("foo", ""));
                }),
            )
            .layer(CookieLayer)
            .boxed()
    }

    async fn body_string(body: BoxBody) -> String {
        let bytes = hyper::body::to_bytes(body).await.unwrap();
        String::from_utf8_lossy(&bytes).into()
    }

    #[tokio::test]
    async fn read_cookies() {
        let req = Request::builder()
            .uri("/list")
            .header(header::COOKIE, "foo=1; bar=2")
            .body(Body::empty())
            .unwrap();
        let res = app().oneshot(req).await.unwrap();
        assert_eq!(body_string(res.into_body()).await, "bar=2, foo=1");
    }

    #[tokio::test]
    async fn add_cookies() {
        let req = Request::builder()
            .uri("/add")
            .header(header::COOKIE, "foo=1; bar=2")
            .body(Body::empty())
            .unwrap();
        let res = app().oneshot(req).await.unwrap();
        let mut hdrs: Vec<_> = res.headers().get_all(header::SET_COOKIE).iter().collect();
        hdrs.sort();
        assert_eq!(hdrs, ["baz=3", "spam=4"]);
    }

    #[tokio::test]
    async fn remove_cookies() {
        let req = Request::builder()
            .uri("/remove")
            .header(header::COOKIE, "foo=1; bar=2")
            .body(Body::empty())
            .unwrap();
        let res = app().oneshot(req).await.unwrap();
        let mut hdrs = res.headers().get_all(header::SET_COOKIE).iter();
        let hdr = hdrs.next().unwrap().to_str().unwrap();
        assert!(hdr.starts_with("foo=; Max-Age=0"));
        assert_eq!(hdrs.next(), None);
    }
}
