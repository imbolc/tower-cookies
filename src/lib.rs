//! A cookie manager middleware built on top of [tower].
//!
//! # Example
//!
//! With [axum]:
//!
//! ```rust,no_run
//! use axum::{routing::get, Router};
//! use std::net::SocketAddr;
//! use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
//!
//! # #[cfg(feature = "axum-core")]
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new()
//!         .route("/", get(handler))
//!         .layer(CookieManagerLayer::new());
//!
//!     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//!     axum::Server::bind(&addr)
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! # #[cfg(not(feature = "axum-core"))]
//! # fn main() {}
//!
//! async fn handler(cookies: Cookies) -> &'static str {
//!     cookies.add(Cookie::new("hello_world", "hello_world"));
//!
//!     "Check your cookies."
//! }
//! ```
//!
//! A complete CRUD cookie example in [examples/counter.rs][example]
//!
//! [axum]: https://crates.io/crates/axum
//! [tower]: https://crates.io/crates/tower
//! [example]: https://github.com/imbolc/tower-cookies/blob/main/examples/counter.rs

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use cookie::CookieJar;
use http::HeaderValue;
use parking_lot::Mutex;
use std::sync::Arc;

#[doc(inline)]
pub use self::service::{CookieManager, CookieManagerLayer};

#[cfg(feature = "signed")]
pub use self::signed::SignedCookies;

#[cfg(feature = "private")]
pub use self::private::PrivateCookies;

#[cfg(any(feature = "signed", feature = "private"))]
pub use cookie::Key;

pub use cookie::Cookie;

#[cfg(feature = "axum-core")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum-core")))]
mod extract;

#[cfg(feature = "signed")]
mod signed;

#[cfg(feature = "private")]
mod private;

pub mod service;

/// A parsed on-demand cookie jar.
#[derive(Clone, Debug, Default)]
pub struct Cookies {
    inner: Arc<Mutex<Inner>>,
}

impl Cookies {
    fn new(headers: Vec<HeaderValue>) -> Self {
        let inner = Inner {
            headers,
            ..Default::default()
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    /// Adds [`Cookie`] to this jar. If a [`Cookie`] with the same name already exists, it is
    /// replaced with provided cookie.
    pub fn add(&self, cookie: Cookie<'static>) {
        let mut inner = self.inner.lock();
        inner.changed = true;
        inner.jar().add(cookie);
    }

    /// Returns the [`Cookie`] with the given name. Returns [`None`] if it doesn't exist.
    pub fn get(&self, name: &str) -> Option<Cookie> {
        let mut inner = self.inner.lock();
        inner.jar().get(name).cloned()
    }

    /// Removes [`Cookie`] from this jar.
    pub fn remove(&self, name: impl Into<CookieName>) {
        let mut inner = self.inner.lock();
        inner.changed = true;
        let cookie = name.into().into();
        inner.jar().remove(cookie);
    }

    /// Returns all the [`Cookie`]s present in this jar.
    ///
    /// This method collects [`Cookie`]s into a vector instead of iterating through them to
    /// minimize the mutex locking time.
    pub fn list(&self) -> Vec<Cookie> {
        let mut inner = self.inner.lock();
        inner.jar().iter().cloned().collect()
    }

    /// Returns a child [`SignedCookies`] jar for interations with signed by the `key` cookies.
    ///
    /// # Example:
    /// ```
    /// use cookie::{Cookie, Key};
    /// use tower_cookies::Cookies;
    ///
    /// let cookies = Cookies::default();
    /// let key = Key::generate();
    /// let signed = cookies.signed(&key);
    ///
    /// let foo = Cookie::new("foo", "bar");
    /// signed.add(foo.clone());
    ///
    /// assert_eq!(signed.get("foo"), Some(foo.clone()));
    /// assert_ne!(cookies.get("foo"), Some(foo));
    /// ```
    #[cfg(feature = "signed")]
    pub fn signed<'a>(&self, key: &'a cookie::Key) -> SignedCookies<'a> {
        SignedCookies::new(self, key)
    }

    /// Returns a child [`PrivateCookies`] jar for encrypting and decrypting cookies.
    ///
    /// # Example:
    /// ```
    /// use cookie::{Cookie, Key};
    /// use tower_cookies::Cookies;
    ///
    /// let cookies = Cookies::default();
    /// let key = Key::generate();
    /// let private = cookies.private(&key);
    ///
    /// let foo = Cookie::new("foo", "bar");
    /// private.add(foo.clone());
    ///
    /// assert_eq!(private.get("foo"), Some(foo.clone()));
    /// assert_ne!(cookies.get("foo"), Some(foo));
    /// ```
    #[cfg(feature = "private")]
    pub fn private<'a>(&self, key: &'a cookie::Key) -> PrivateCookies<'a> {
        PrivateCookies::new(self, key)
    }
}

#[derive(Debug, Default)]
struct Inner {
    headers: Vec<HeaderValue>,
    jar: Option<CookieJar>,
    changed: bool,
}

impl Inner {
    fn jar(&mut self) -> &mut CookieJar {
        if self.jar.is_none() {
            let mut jar = CookieJar::new();
            for header in &self.headers {
                if let Ok(header_str) = std::str::from_utf8(header.as_bytes()) {
                    for cookie_str in header_str.split(';') {
                        if let Ok(cookie) = cookie::Cookie::parse_encoded(cookie_str.to_owned()) {
                            jar.add_original(cookie);
                        }
                    }
                }
            }
            self.jar = Some(jar);
        }
        self.jar.as_mut().unwrap()
    }
}

/// Something we can transform into a cookie name
#[allow(clippy::large_enum_variant)]
pub enum CookieName {
    /// Cookie name as `&'static string`
    Str(&'static str),
    /// Cookie name as owned `String`
    String(String),
    /// Cookie instance
    Cookie(Cookie<'static>),
}

impl From<CookieName> for Cookie<'static> {
    fn from(src: CookieName) -> Self {
        match src {
            CookieName::Str(s) => Cookie::new(s, ""),
            CookieName::String(s) => Cookie::new(s, ""),
            CookieName::Cookie(c) => c,
        }
    }
}

impl From<&'static str> for CookieName {
    fn from(src: &'static str) -> Self {
        Self::Str(src)
    }
}

impl From<String> for CookieName {
    fn from(src: String) -> Self {
        Self::String(src)
    }
}

impl From<Cookie<'static>> for CookieName {
    fn from(src: Cookie<'static>) -> Self {
        Self::Cookie(src)
    }
}

#[cfg(all(test, feature = "axum-core"))]
mod tests {
    use crate::{CookieManagerLayer, Cookies};
    use axum::{
        body::{Body, BoxBody},
        routing::get,
        Router,
    };
    use cookie::Cookie;
    use http::{header, Request};
    use tower::ServiceExt;

    fn app() -> Router {
        Router::new()
            .route(
                "/list",
                get(|cookies: Cookies| async move {
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
                get(|cookies: Cookies| async move {
                    cookies.add(Cookie::new("baz", "3"));
                    cookies.add(Cookie::new("spam", "4"));
                }),
            )
            .route(
                "/remove",
                get(|cookies: Cookies| async move {
                    cookies.remove(Cookie::new("foo", ""));
                    cookies.remove("bar");
                    cookies.remove(String::from("baz"));
                }),
            )
            .layer(CookieManagerLayer::new())
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
    async fn read_multi_header_cookies() {
        let req = Request::builder()
            .uri("/list")
            .header(header::COOKIE, "foo=1")
            .header(header::COOKIE, "bar=2")
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
        use std::collections::hash_set::HashSet;

        let req = Request::builder()
            .uri("/remove")
            .header(header::COOKIE, "foo=1; bar=2; baz=3")
            .body(Body::empty())
            .unwrap();
        let res = app().oneshot(req).await.unwrap();
        let hdrs = res.headers().get_all(header::SET_COOKIE).iter();
        let hdrs: HashSet<_> = hdrs.map(|h| h.to_str().unwrap()[..15].to_owned()).collect();
        let expected = ["foo=; Max-Age=0", "bar=; Max-Age=0", "baz=; Max-Age=0"]
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(hdrs, expected);
    }
}
