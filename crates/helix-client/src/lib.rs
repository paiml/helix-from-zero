//! Typed Rust client for HelixDB HTTP endpoints.
//!
//! The companion to course 20 ("HelixDB from Zero") in the Rust for Data
//! Engineering specialization. Wraps the HTTP API exposed by `helix push
//! dev` so a Rust binary can call HelixQL `QUERY`s by name with typed
//! parameters and typed return values.
//!
//! The client is intentionally minimal — every QUERY in `db/queries.hx`
//! becomes one `async fn` here. The lesson 5.1.3 demo uses
//! [`HelixClient::list_top_films`] to walk through the typed pattern and
//! the four runtime contracts that ship in `contracts/helix-rust-v1.yaml`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod client;
mod contracts;
mod error;
mod types;

pub use client::HelixClient;
pub use contracts::{
    assert_all_ids_positive, assert_descending_sort, assert_row_count,
    assert_top_score_non_negative, ContractViolation,
};
pub use error::{Error, Result};
pub use types::{FilmHit, ListTopFilmsParams};
