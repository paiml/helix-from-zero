//! Typed request and response shapes for the HelixQL endpoints we call.
//!
//! Every QUERY in `db/queries.hx` that the Rust client demo touches has
//! its parameter and return shape pinned here so rustc rejects mismatched
//! calls before the binary ever runs.

use serde::{Deserialize, Serialize};

/// Parameters for the `listTopFilms` HelixQL query.
#[derive(Debug, Clone, Serialize)]
pub struct ListTopFilmsParams {
    /// Source label that the films are filtered on (Doc.source field).
    pub genre: String,
    /// Maximum number of films to return.
    pub limit: i64,
}

/// One row returned by `listTopFilms` (and other QUERY endpoints that
/// return Doc-shaped rows).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FilmHit {
    /// HelixDB ID (positive integer per the schema).
    pub id: i64,
    /// Document title.
    pub title: String,
    /// Document content (the row's full text).
    pub content: String,
    /// Document source label.
    pub source: String,
    /// Similarity or rank score in [0, 1].
    #[serde(default)]
    pub score: f64,
}
