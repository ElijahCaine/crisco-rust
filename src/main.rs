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
#![feature(stmt_expr_attributes)]
#[deny(missing_docs)]

extern crate iron;

pub mod database;
pub mod handler;
pub mod routes;

fn main() { }
