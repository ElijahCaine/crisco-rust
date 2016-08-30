//! # Crisco: A URL Shortening Webapp
//!
//! Thank you for having an interest in Crisco. The project is in it's very early stages. If you
//! would like to contribute get in contact with `pop` on `irc.mozilla.org`.
//!
//! TODO:
//!
//! - Start Server
//!

#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/en/4/47/Crisco_logo.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico")]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![feature(stmt_expr_attributes, custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]
#[deny(missing_docs)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate router;
extern crate dotenv;
extern crate iron;
extern crate liquid;

pub mod database;
pub mod routes;

// use iron::Iron;


fn main() {
    //     // Figure out where to put this stuff
    //     let router = router!(index:   get     "/"       => routes::index,
    //                          query:   get     "/:query" => routes::get,
    //                          post:    put     "/"       => routes::put,
    //                          delete:  delete  "/:query" => routes::del);
    //
    //     match Iron::new(router).http("localhost:3000") {
    //         Ok(_) => println!("Starting server on localhost:3000"),
    //         Err(_) => println!("Failed to start server. Exiting"),
    //     };
}
