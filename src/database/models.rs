//! # Models
//!
//! The database models for interfacing with the `diesel` ORM.
//!
use super::schema::urls;

#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub path: String,
    pub dest: String,
}

#[insertable_into(urls)]
pub struct NewUrl<'a> {
    pub path: &'a str,
    pub dest: &'a str,
}
