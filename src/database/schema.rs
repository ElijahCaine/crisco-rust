//! # Schema
//!
//! The schema file for `crisco`.
//!
//! Previously used `infer_schema!` macro, not uses `table!` macro for in-memory database
//! migrations.
//!

table! {
    urls {
        id   -> Integer,
        path -> VarChar,
        dest -> VarChar,
    }
}
