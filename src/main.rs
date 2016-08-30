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

use database::{insert_url, connect, migrate, latest_url, get_url};
use database::models::NewUrl;

use diesel::sqlite::SqliteConnection;


fn main() {
    let conn = connect(true);
    match migrate(&conn) {
        Ok(()) => test_populate_db(&conn),
        Err(_) => println!("Oh shit! There was an error running migrations."),
    }
}

fn test_populate_db(conn: &SqliteConnection) {
    let new_urls = vec![
        NewUrl { path: "a", dest: "1" },
        NewUrl { path: "b", dest: "2" },
        NewUrl { path: "c", dest: "3" },
        NewUrl { path: "a", dest: "1" },
    ];

    for url in new_urls {
        match insert_url(conn, &url) {
            Some(t) => println!("id: {:?} | \t path: {:?} | \t dest: {:?}", t.id, t.path, t.dest),
            None    => println!("duplicate: \t path: {:?} | \t dest: {:?}", url.path, url.dest),
        }
    }

    match latest_url(conn) {
        Some(t) => println!("Latest Url: id {:?} | \t path: {:?} | \t dest: {:?}",
                                            t.id,          t.path,         t.dest),
        None    => println!("No latest entry?"),
    }

    let path = "a";
    match get_url(conn, &path.to_string()) {
        Some(t) => println!("Entry `a`:  id {:?} | \t path: {:?} | \t dest: {:?}",
                                            t.id,          t.path,         t.dest),
        None    => println!("Couldn't fine entry corresponding to {}", path),
    }
}
