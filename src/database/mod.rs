//!
//! # Database Lib
//!
//! Crisco uses the `diesel` database ORM to interface with a `sqlite` database.
//!
//! ## TODO:
//!
//! - `get_entry` *high*
//! - `delete_entry` *low*
//! - `update_entry` *low*
//! - `migrate` *medium*
//! - Use transactions *low*
//!
// #![feature(custom_derive, custom_attribute, plugin)]
// #![plugin(diesel_codegen, dotenv_macros)]

extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use self::models::{Url, NewUrl};

use self::schema::urls;

use diesel::{ExecuteDsl, LoadDsl};
use diesel::connection::Connection;
use diesel::sqlite::SqliteConnection;

use dotenv::dotenv;

use std::env;


///
/// Establish a connection to the database at DATABASE_URL.
///
pub fn connect(mem: bool) -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
                     .expect("DATABASE_URL must be set.");

    if mem {
        SqliteConnection::establish(":memory:").unwrap()
    } else {
        SqliteConnection::establish(&db_url)
                         .expect(&format!("Error connecting to {}.", db_url))
    }
}

///
/// Add entry to database.
///
/// Returns Url entered into database.
///
pub fn insert_url<'a>(conn: &SqliteConnection, entry: &NewUrl) -> Option<Url> {
    // Insert returns an Option so we need to handle that properly.
    match diesel::insert(entry)
                 .into(urls::table)
                 .execute(conn)
    {
        // Load also returns an option but that's not liable to fail like insert does.
        Ok(_) => urls::table.load(conn)
                            .unwrap()
                            .pop(),
        Err(_) => None
    }
}

///
/// Returns latest entry added to the database.
///
pub fn latest_url(conn: &SqliteConnection) -> Option<Url> {
}
