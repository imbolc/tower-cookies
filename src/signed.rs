use crate::Cookies;
use cookie::{Cookie, Key};

/// A child cookie jar that authenticates its cookies.
/// It signs all the cookies added to it and verifies cookies retrieved from it.
/// Any cookies stored in `SignedCookies` are provided integrity and authenticity. In other
/// words, clients cannot tamper with the contents of a cookie nor can they fabricate cookie
/// values, but the data is visible in plaintext.
pub struct SignedCookies<'a> {
    cookies: Cookies,
    key: &'a Key,
}

impl<'a> SignedCookies<'a> {
    /// Creates an instance of `SignedCookies` with parent `cookies` and key `key`. This method is
    /// typically called indirectly via the `signed` method of [`Cookies`].
    pub(crate) fn new(cookies: &Cookies, key: &'a Key) -> Self {
        Self {
            cookies: cookies.clone(),
            key,
        }
    }

    /// Adds cookie to the parent jar. The cookie’s value is signed assuring integrity and
    /// authenticity.
    pub fn add(&self, cookie: Cookie<'static>) {
        let mut inner = self.cookies.inner.lock();
        inner.changed = true;
        inner.jar().signed_mut(self.key).add(cookie);
    }

    /// Returns `Cookie` with the `name` and verifies the authenticity and integrity of the
    /// cookie’s value, returning a `Cookie` with the authenticated value. If the cookie cannot be
    /// found, or the cookie fails to verify, None is returned.
    pub fn get(&self, name: &str) -> Option<Cookie<'static>> {
        let mut inner = self.cookies.inner.lock();
        inner.jar().signed(self.key).get(name)
    }

    /// Removes the `cookie` from the parent jar.
    pub fn remove(&self, cookie: Cookie<'static>) {
        self.cookies.remove(cookie);
    }
}

#[cfg(all(test, feature = "signed"))]
mod tests {
    use crate::Cookies;
    use cookie::{Cookie, Key};

    #[test]
    fn get_absent() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        assert_eq!(cookies.signed(&key).get("foo"), None);
    }

    #[test]
    fn add_get_signed() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        let signed = cookies.signed(&key);
        signed.add(cookie.clone());
        assert_eq!(signed.get("foo").unwrap(), cookie);
    }

    #[test]
    fn add_signed_get_raw() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        cookies.signed(&key).add(cookie.clone());
        assert_ne!(cookies.get("foo").unwrap(), cookie);
    }

    #[test]
    fn add_raw_get_signed() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        cookies.add(cookie);
        assert_eq!(cookies.signed(&key).get("foo"), None);
    }

    #[test]
    fn messed_keys() {
        let key1 = Key::generate();
        let key2 = Key::generate();
        let cookies = Cookies::new(vec![]);
        let cookie = Cookie::new("foo", "bar");
        cookies.signed(&key1).add(cookie);
        assert_eq!(cookies.signed(&key2).get("foo"), None);
    }

    #[test]
    fn remove() {
        let key = Key::generate();
        let cookies = Cookies::new(vec![]);
        let signed = cookies.signed(&key);
        signed.add(Cookie::new("foo", "bar"));
        let cookie = signed.get("foo").unwrap();
        signed.remove(cookie);
        assert!(signed.get("foo").is_none());
    }
}
