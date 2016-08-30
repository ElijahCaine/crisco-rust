//! # Database Lib
//!
//! Crisco uses the `diesel` database ORM to interface with a `sqlite` database.
//!
//! ## TODO:
//!
//! - `delete_entry` *low*
//! - `update_entry` *low*
//! - Use transactions *low*
//!
//! ## Usage
//!
//! includes:
//!
//! ```rust
//! use database::{put_url, connect, migrate, latest_url, get_url};
//! use database::models::NewUrl;
//! use diesel::sqlite::SqliteConnection;
//! ```
//!
//! Connect to in-memory database and migrate:
//!
//! ```rust
//! let conn = connect(true);
//! match migrate(&conn) {
//!     Ok(()) => test_populate_db(&conn),
//!     Err(_) => println!("Oh shit! There was an error running migrations."),
//! }
//! ```
//!
//! Populate database:
//!
//! ```rust
//! fn test_populate_db(conn: &SqliteConnection) {
//!     let new_urls = vec![
//!         NewUrl { path: "a", dest: "1" },
//!         NewUrl { path: "b", dest: "2" },
//!         NewUrl { path: "c", dest: "3" },
//!         NewUrl { path: "a", dest: "1" },
//!     ];
//!
//!     for url in new_urls {
//!         match put_url(conn, &url) {
//!             Some(t) => println!("id: {:?} | \t path: {:?} | \t dest: {:?}",
//!                                      t.id,          t.path,         t.dest),
//!             None    => println!("duplicate: \t path: {:?} | \t dest: {:?}",
//!                                                    url.path,       url.dest),
//!         }
//!     }
//!
//!     match latest_url(conn) {
//!         Some(t) => println!("Latest Url: id {:?} | \t path: {:?} | \t dest: {:?}",
//!                                             t.id,          t.path,         t.dest),
//!         None    => println!("No latest entry?"),
//!     }
//!
//!     let path = "a";
//!     match get_url(conn, &path.to_string()) {
//!         Some(t) => println!("Entry `a`:  id {:?} | \t path: {:?} | \t dest: {:?}",
//!                                             t.id,          t.path,         t.dest),
//!         None    => println!("Couldn't fine entry corresponding to {}", path),
//!     }
//! }
//! ```
//!
//! #![feature(custom_derive, custom_attribute, plugin)]
//! #![plugin(diesel_codegen, dotenv_macros)]

extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use self::models::{Url, NewUrl};

use self::schema::urls::dsl::*;

use diesel::{ExecuteDsl, LoadDsl, FilterDsl, ExpressionMethods, LimitDsl};
use diesel::connection::Connection;
use diesel::sqlite::SqliteConnection;
use diesel::migrations::{run_pending_migrations, RunMigrationsError};

use dotenv::dotenv;

use std::env;


///
/// Establish a connection to the database at DATABASE_URL.
///
pub fn connect(mem: bool) -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    if mem {
        SqliteConnection::establish(":memory:").unwrap()
    } else {
        SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}.", db_url))
    }
}


///
/// Runs migrations on database.
///
pub fn migrate(conn: &SqliteConnection) -> Result<(), RunMigrationsError> {
    run_pending_migrations(conn)
}


///
/// Add entry to database.
///
/// Returns Url entered into database.
///
pub fn put_url(conn: &SqliteConnection, entry: &NewUrl) -> Option<Url> {
    // Insert returns an Option so we need to handle that properly.
    match diesel::insert(entry)
        .into(urls)
        .execute(conn) {
        // Load also returns an option but that's not liable to fail like insert does.
        // We should be able to use diesel's `get_result` but we can't so this has to suffice.
        Ok(_) => latest_url(conn),
        Err(_) => None,
    }
}


///
/// Get a URL from the database.
///
/// Returns a Url given a Path (`p: &String`).
///
pub fn get_url(conn: &SqliteConnection, p: &String) -> Option<Url> {
    // Grab all urls with this path (should only be one).
    // Limit the pool to 1 items.
    // This should be true by default, might cause bugs(?).
    // Unwrap the Result to get the Vec.
    // Pop the (hopefully single) element out of the Vec.

    urls.filter(path.eq(p))
        .limit(1)
        .load(conn)
        .unwrap()
        .pop()
}


///
/// Get the latest URL from the database.
///
/// Returns latest URL added to the database given a connection.
///
pub fn latest_url(conn: &SqliteConnection) -> Option<Url> {
    // Load all elements from the database.
    // Unwrap the Result into a Vec.
    // Pop the last element off of the Vec.

    // TODO: Optimize this
    urls.load(conn)
        .unwrap()
        .pop()
}

// ///
// /// Deletes a specific URL from the database.
// ///
// pub fn delete_url(conn: &SqliteConnection, p: &String) -> Result<Url, result::Error> {}


// #[cfg(tests)]
// mod tests {
//     use super::{connect, migrate, put_url, get_url, latest_url, del_url};
//
//     #[test]
//     fn connect_works() {}
//
//     #[test]
//     fn migrate_works() {}
//
//     #[test]
//     fn put_url_works() {}
//
//     #[test]
//     fn latest_url_works() {}
//
//     #[test]
//     fn del_url_works() {}
// }
