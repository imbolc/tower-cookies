use crate::Cookies;
use cookie::{Cookie, Key};

/// A cookie jar that provides authenticated encryption for its cookies.
///
/// A _private_ child jar signs and encrypts all the cookies added to it and
/// verifies and decrypts cookies retrieved from it. Any cookies stored in
/// `PrivateCookies` are simultaneously assured confidentiality, integrity, and
/// authenticity. In other words, clients cannot discover nor tamper with the
/// contents of a cookie, nor can they fabricate cookie data.
pub struct PrivateCookies<'a> {
    cookies: Cookies,
    key: &'a Key,
}

impl<'a> PrivateCookies<'a> {
    /// Creates an instance of `PrivateCookies` with parent `cookies` and key `key`.
    /// This method is typically called indirectly via the `private`
    /// method of [`Cookies`].
    pub(crate) fn new(cookies: &Cookies, key: &'a Key) -> Self {
        Self {
            cookies: cookies.clone(),
            key,
        }
    }

    /// Adds `cookie` to the parent jar. The cookie's value is encrypted with
    /// authenticated encryption assuring confidentiality, integrity, and
    /// authenticity.
    pub fn add(&self, cookie: Cookie<'static>) {
        let mut inner = self.cookies.inner.lock();
        inner.changed = true;
        inner.jar().private_mut(self.key).add(cookie);
    }

    /// Returns a reference to the `Cookie` inside this jar with the name `name`
    /// and authenticates and decrypts the cookie's value, returning a `Cookie`
    /// with the decrypted value. If the cookie cannot be found, or the cookie
    /// fails to authenticate or decrypt, `None` is returned.
    pub fn get(&self, name: &str) -> Option<Cookie<'static>> {
        let mut inner = self.cookies.inner.lock();
        inner.jar().private(self.key).get(name)
    }

    /// Removes the `cookie` from the parent jar.
    pub fn remove(&self, cookie: Cookie<'static>) {
        self.cookies.remove(cookie);
    }
}

#[cfg(all(test, feature = "private"))]
mod tests {
    use crate::Cookies;
    use cookie::{Cookie, Key};

    #[test]
    fn get_absent() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        assert_eq!(cookies.private(&key).get("foo"), None);
    }

    #[test]
    fn add_get_private() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        let private = cookies.private(&key);
        private.add(cookie.clone());
        assert_eq!(private.get("foo").unwrap(), cookie);
    }

    #[test]
    fn add_private_get_raw() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        cookies.private(&key).add(cookie.clone());
        assert_ne!(cookies.get("foo").unwrap(), cookie);
    }

    #[test]
    fn add_raw_get_private() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        cookies.add(cookie);
        assert_eq!(cookies.private(&key).get("foo"), None);
    }

    #[test]
    fn messed_keys() {
        let key1 = Key::generate();
        let key2 = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        cookies.private(&key1).add(cookie);
        assert_eq!(cookies.private(&key2).get("foo"), None);
    }

    #[test]
    fn remove() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let private = cookies.private(&key);
        private.add(Cookie::new("foo", "bar"));
        let cookie = private.get("foo").unwrap();
        private.remove(cookie);
        assert!(private.get("foo").is_none());
    }
}
