//!
//! # Schema
//!
//! The schema file for `crisco`. Theoretically very simple.
//!

// infer_schema!(dotenv!("DATABASE_URL"));

table! {
    urls {
        id   -> Integer,
        path -> VarChar,
        dest -> VarChar,
    }
}
