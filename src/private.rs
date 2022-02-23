use crate::Cookies;
use cookie::{Cookie, Key};

/// A child cookie jar that encrypts its cookies.
pub struct PrivateCookies<'a> {
    cookies: Cookies,
    key: &'a Key,
}

impl<'a> PrivateCookies<'a> {
    /// Creates an instance of `PrivateCookies` with parent `cookies` and key `key`. This method is
    /// typically called indirectly via the `private` method of [`Cookies`].
    pub(crate) fn new(cookies: &Cookies, key: &'a Key) -> Self {
        Self {
            cookies: cookies.clone(),
            key,
        }
    }

    /// Adds cookie to the parent jar. The cookie’s value is encrypted.
    pub fn add(&self, cookie: Cookie<'static>) {
        let mut inner = self.cookies.inner.lock();
        inner.changed = true;
        inner.jar().private_mut(self.key).add(cookie);
    }

    /// Returns `Cookie` with the `name` and decrypted contents.
    pub fn get(&self, name: &str) -> Option<Cookie> {
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
        let cookie = Cookie::new("foo", "bar");
        let private = cookies.private(&key);
        private.add(cookie.clone());
        assert!(private.get("foo").is_some());
        private.remove(cookie);
        assert!(private.get("foo").is_none());
    }
}
