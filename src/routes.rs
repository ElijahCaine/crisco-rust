//! # Routes
//!
//! The routing handlers for `crisco`.
//!
//! TODO:
//!
//! - `index`
//! - `get`
//! - `put`
//! - `del`
//!
//! use iron::{Request, Response, IronResult, status};

// use liquid::{Renderable, Context, Value};

// use router::Router;

// pub fn handler(req: &mut Request) -> IronResult<Response> {
//     let ref q = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
//
//     if *q == "/" {
//         println!("Index Page");
//     }
//
//     Ok(Response::with((status::Ok, *q)))
// }

// ///
// /// Basic index handler.
// ///
// /// Returns the homepage or 'success' page if redirected from `put`.
// ///
// pub fn index(req: &mut Request) -> IronResult<Response> {
//     // let ref q = req.extensions
//     //                .get::<Router>()
//     //                .unwrap()
//     //                .find("query")
//     //                .unwrap_or("/");
//     // Compile query parameters into data (Alerts, Status, Copy-able URl, etc)
//     // Send data to template
//     // Render template
//     // Send response
// }
//
// ///
// /// Get handler
// ///
// /// Redirects from /:query to destination website.
// /// Redirects to index page with a 404 status alert if not found.
// ///
// pub fn get(req: &mut Request) -> IronResult<Response> {
//     // Parse "query" variable
//     // Check database for query Url
//     // If it exists
//     //   Return 302 status
//     //   forward to that URL
//     // Else
//     //   Return 404 status
//     //   render index page
// }
//
// ///
// /// Put handler
// ///
// /// Adds a new Url to the database.
// /// Redirects to index page with a status alert.
// ///
// pub fn put(req: &mut Request) -> IronResult<Response> {
//     // Parse query parameters for 'path' and 'dest'.
//     // If 'path' isn't already taken,
//     //    add URL to the database
//     // else
//     //   return 409 (or 403)
//     //   render index page with status/copy-able URL.
// }
//
// ///
// /// Del handler
// ///
// /// Removes item from database.
// /// Redirects to index page with a status alert.
// ///
// pub fn del(req: &mut Request) -> IronResult<Response> {
//     // Parse "query" variable
//     // Check database for query Url
//     // if it exists
//     //   Remove from database
//     //   Return 204 (or 200)
//     //   Render index page with status
// }
