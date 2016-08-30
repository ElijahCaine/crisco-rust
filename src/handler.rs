//!
//! # URL Handler
//!
//! The custom url handler for the `iron` library.
//!
//! It looks a lot like [iron's simple routing example][1].
//!
//! [1]: https://github.com/iron/iron/blob/master/examples/simple_routing.rs
//!
//! ## Example:
//!
//! ```rust
//! let mut router = Router::new();
//! router.add("endpoint".to_string(), |_: &mut Request| {
//!     Ok(Response::with((status::Ok, "Lorem Ipsum")))
//! });
//! ```
//!
//! ## TODO:
//! method:
//! - `contains` method
//! testing:
//! - `contains` method
//!
//!
extern crate iron;

use std::collections::HashMap;

use iron::prelude::{Response, Request, IronResult};
use iron::{Handler, status};

///
/// The `Router` data-type.
///
pub struct Router {
    ///
    /// Main data is HashMap of String to Response.
    ///
    routes: HashMap<String, Box<Handler>>
}

///
/// Router methods.
///
impl Router {
    ///
    /// Create new handler.
    ///
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    ///
    /// Add new route to handler.
    ///
    pub fn add<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }

    ///
    /// Delete a route from the router.
    ///
    pub fn del(&mut self, path: &String) -> bool {
        match self.routes.remove(path) {
            Some(_) => return true,
            None => return false
        }
    }
}

///
/// `Handler` implementation on Router.
///
impl Handler for Router {
    ///
    /// Handles a given URL path.
    ///
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Router;
    use super::iron;

    #[test]
    fn add_works() {
        let mut router = Router::new();
        router.add("example".to_string(), |_: &mut iron::prelude::Request| {
            Ok(iron::prelude::Response::with(iron::status::Ok))
        });

        /// Handler is not empty after adding an entry.
        assert_eq!(router.routes.is_empty(), false);

        /// Handler contains the 'endpoint' entry.
        assert_eq!(router.routes.contains_key("example"), true);

        /// Handler does not contain the 'other_endpoint' entry.
        assert_eq!(router.routes.contains_key("other_endpoint"), false);
    }

    #[test]
    fn del_works() {
        let mut router = Router::new();
        router.add("example".to_string(), |_: &mut iron::prelude::Request| {
            Ok(iron::prelude::Response::with(iron::status::Ok))
        });

        /// Handler is not empty after adding an entry.
        assert_eq!(router.routes.is_empty(), false);

        /// Handler contains the 'example' entry after adding it.
        assert_eq!(router.routes.contains_key("example"), true);

        let v1 = router.del(&"example".to_string());

        /// Handler returns true after successful deleion.
        assert_eq!(v1, true);

        /// Handler is empty after deleting an entry.
        assert_eq!(router.routes.is_empty(), true);

        /// Handler does not contain 'example' entry after deletion.
        assert_eq!(router.routes.contains_key("example"), false);

        let v2 = router.del(&"example".to_string());
        /// Handler returns false after un-successful deleion.
        assert_eq!(v2, false);
    }
}
