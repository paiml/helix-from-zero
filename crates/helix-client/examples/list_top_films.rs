//! Lesson 5.1.3 — Typed Rust Client demo.
//!
//! Calls the `listTopFilms` HelixQL endpoint on `helix push dev` (default
//! port 6969) and asserts the four runtime contracts from
//! `contracts/helix-rust-v1.yaml`. Designed to fail loudly when the
//! endpoint shape drifts away from the typed `FilmHit` struct.
//!
//! Run with:
//!
//! ```bash
//! HELIX_URL=http://localhost:6969 \
//!   cargo run -p helix-client --example list_top_films
//! ```

use helix_client::{
    assert_all_ids_positive, assert_descending_sort, assert_row_count,
    assert_top_score_non_negative, HelixClient,
};

const DEFAULT_HELIX_URL: &str = "http://localhost:6969";
const GENRE: &str = "Comedy";
const TOP_K: i64 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("HELIX_URL").unwrap_or_else(|_| DEFAULT_HELIX_URL.into());
    let client = HelixClient::new(url);

    let hits = client.list_top_films(GENRE, TOP_K).await?;

    assert_row_count(&hits, TOP_K as usize)?;
    assert_top_score_non_negative(&hits)?;
    assert_all_ids_positive(&hits)?;
    assert_descending_sort(&hits)?;

    println!("{}", serde_json::to_string_pretty(&hits)?);
    Ok(())
}
