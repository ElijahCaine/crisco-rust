//!
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
extern crate dotenv;
extern crate iron;

pub mod database;
pub mod handler;
pub mod routes;

use database::{insert_url, connect};
use database::models::NewUrl;


fn main() {
    let conn = connect(false);

    let new_urls = vec![
        NewUrl { path: "a", dest: "1" },
        NewUrl { path: "b", dest: "2" },
        NewUrl { path: "c", dest: "3" },
        NewUrl { path: "a", dest: "1" },
    ];

    for url in new_urls {
        match insert_url(&conn, &url) {
            Some(t) => println!("id: {:?} | path: {:?} | dest: {:?}", t.id, t.path, t.dest),
            None    => println!("duplicate: path: {:?} | dest: {:?}", url.path, url.dest),
        }
    }
}
