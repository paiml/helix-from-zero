//! Provable runtime contracts for the lesson 5.1.3 demo.
//!
//! Each function returns `Ok(())` when the contract holds and a typed
//! [`ContractViolation`] when it does not. Pulling them out as pure
//! functions keeps the `examples/list_top_films.rs` body short — the
//! example calls every function with `?` — and lets the unit tests
//! exercise every branch directly.
//!
//! Mirrors the four obligations in `contracts/helix-rust-v1.yaml`:
//!
//! 1. `hits.len() == k` — top-k honoured.
//! 2. `hits[0].score >= 0.0` — top score non-negative.
//! 3. `forall h. h.id > 0` — every film id is a positive integer.
//! 4. `forall i. hits[i].score >= hits[i+1].score` — pairwise descending.

use thiserror::Error;

use crate::FilmHit;

/// A contract obligation that the response failed to satisfy.
#[derive(Debug, Error, PartialEq)]
pub enum ContractViolation {
    /// Returned row count differs from the requested top-k.
    #[error("row count contract: expected {expected} hits, got {actual}")]
    RowCount {
        /// Expected number of hits (the `k` argument to `listTopFilms`).
        expected: usize,
        /// Actual number of hits the helix instance returned.
        actual: usize,
    },

    /// Top hit's score is negative — should be in [0, 1].
    #[error("top score contract: top score is negative ({0})")]
    TopScoreNegative(f64),

    /// One of the hits has a non-positive id — schema invariant says id > 0.
    #[error("id positivity contract: hit at index {index} has non-positive id {id}")]
    IdNonPositive {
        /// Position of the offending hit in the response slice.
        index: usize,
        /// Offending id value.
        id: i64,
    },

    /// Hits are not pairwise sorted by descending score.
    #[error(
        "descending sort contract: hits[{index}].score ({lhs}) < hits[{}].score ({rhs})",
        .index + 1
    )]
    NotDescending {
        /// Index of the hit whose score is lower than its neighbour.
        index: usize,
        /// Score at `index`.
        lhs: f64,
        /// Score at `index + 1`.
        rhs: f64,
    },
}

/// Contract 1 — the response carries exactly `expected` rows.
pub fn assert_row_count(hits: &[FilmHit], expected: usize) -> Result<(), ContractViolation> {
    if hits.len() == expected {
        Ok(())
    } else {
        Err(ContractViolation::RowCount {
            expected,
            actual: hits.len(),
        })
    }
}

/// Contract 2 — `hits[0].score` is non-negative.
///
/// An empty slice is treated as vacuously satisfying the contract; row
/// count is enforced by [`assert_row_count`] upstream.
pub fn assert_top_score_non_negative(hits: &[FilmHit]) -> Result<(), ContractViolation> {
    match hits.first() {
        Some(h) if h.score < 0.0 => Err(ContractViolation::TopScoreNegative(h.score)),
        _ => Ok(()),
    }
}

/// Contract 3 — every film id is a positive integer.
pub fn assert_all_ids_positive(hits: &[FilmHit]) -> Result<(), ContractViolation> {
    for (index, h) in hits.iter().enumerate() {
        if h.id <= 0 {
            return Err(ContractViolation::IdNonPositive { index, id: h.id });
        }
    }
    Ok(())
}

/// Contract 4 — hits are pairwise sorted by descending score.
pub fn assert_descending_sort(hits: &[FilmHit]) -> Result<(), ContractViolation> {
    for (index, window) in hits.windows(2).enumerate() {
        let lhs = window[0].score;
        let rhs = window[1].score;
        if lhs < rhs {
            return Err(ContractViolation::NotDescending { index, lhs, rhs });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(id: i64, score: f64) -> FilmHit {
        FilmHit {
            id,
            title: format!("Film {id}"),
            content: "fixture".into(),
            source: "Comedy".into(),
            score,
        }
    }

    #[test]
    fn row_count_matches_expected() {
        let hits = vec![hit(1, 0.9), hit(2, 0.8)];
        assert!(assert_row_count(&hits, 2).is_ok());
    }

    #[test]
    fn row_count_too_few_violates() {
        let hits = vec![hit(1, 0.9)];
        assert_eq!(
            assert_row_count(&hits, 5),
            Err(ContractViolation::RowCount {
                expected: 5,
                actual: 1
            })
        );
    }

    #[test]
    fn row_count_too_many_violates() {
        let hits = vec![hit(1, 0.9), hit(2, 0.8), hit(3, 0.7)];
        assert_eq!(
            assert_row_count(&hits, 2),
            Err(ContractViolation::RowCount {
                expected: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn top_score_zero_passes() {
        let hits = vec![hit(1, 0.0)];
        assert!(assert_top_score_non_negative(&hits).is_ok());
    }

    #[test]
    fn top_score_positive_passes() {
        let hits = vec![hit(1, 0.42)];
        assert!(assert_top_score_non_negative(&hits).is_ok());
    }

    #[test]
    fn top_score_empty_passes_vacuously() {
        let hits: Vec<FilmHit> = Vec::new();
        assert!(assert_top_score_non_negative(&hits).is_ok());
    }

    #[test]
    fn top_score_negative_violates() {
        let hits = vec![hit(1, -0.001)];
        assert_eq!(
            assert_top_score_non_negative(&hits),
            Err(ContractViolation::TopScoreNegative(-0.001))
        );
    }

    #[test]
    fn ids_all_positive_passes() {
        let hits = vec![hit(1, 0.9), hit(2, 0.8)];
        assert!(assert_all_ids_positive(&hits).is_ok());
    }

    #[test]
    fn ids_empty_passes_vacuously() {
        let hits: Vec<FilmHit> = Vec::new();
        assert!(assert_all_ids_positive(&hits).is_ok());
    }

    #[test]
    fn ids_zero_violates() {
        let hits = vec![hit(1, 0.9), hit(0, 0.8)];
        assert_eq!(
            assert_all_ids_positive(&hits),
            Err(ContractViolation::IdNonPositive { index: 1, id: 0 })
        );
    }

    #[test]
    fn ids_negative_violates() {
        let hits = vec![hit(-7, 0.9)];
        assert_eq!(
            assert_all_ids_positive(&hits),
            Err(ContractViolation::IdNonPositive { index: 0, id: -7 })
        );
    }

    #[test]
    fn descending_strict_passes() {
        let hits = vec![hit(1, 0.9), hit(2, 0.8), hit(3, 0.5)];
        assert!(assert_descending_sort(&hits).is_ok());
    }

    #[test]
    fn descending_equal_passes() {
        let hits = vec![hit(1, 0.5), hit(2, 0.5)];
        assert!(assert_descending_sort(&hits).is_ok());
    }

    #[test]
    fn descending_empty_passes_vacuously() {
        let hits: Vec<FilmHit> = Vec::new();
        assert!(assert_descending_sort(&hits).is_ok());
    }

    #[test]
    fn descending_single_passes_vacuously() {
        let hits = vec![hit(1, 0.42)];
        assert!(assert_descending_sort(&hits).is_ok());
    }

    #[test]
    fn descending_violation_at_first_pair() {
        let hits = vec![hit(1, 0.5), hit(2, 0.9)];
        assert_eq!(
            assert_descending_sort(&hits),
            Err(ContractViolation::NotDescending {
                index: 0,
                lhs: 0.5,
                rhs: 0.9,
            })
        );
    }

    #[test]
    fn descending_violation_in_middle() {
        let hits = vec![hit(1, 0.9), hit(2, 0.5), hit(3, 0.8)];
        assert_eq!(
            assert_descending_sort(&hits),
            Err(ContractViolation::NotDescending {
                index: 1,
                lhs: 0.5,
                rhs: 0.8,
            })
        );
    }

    #[test]
    fn violation_messages_include_indices() {
        let row = ContractViolation::RowCount {
            expected: 5,
            actual: 3,
        };
        assert!(row.to_string().contains("expected 5"));
        assert!(row.to_string().contains("got 3"));

        let top = ContractViolation::TopScoreNegative(-0.5);
        assert!(top.to_string().contains("-0.5"));

        let id = ContractViolation::IdNonPositive { index: 2, id: -1 };
        assert!(id.to_string().contains("index 2"));
        assert!(id.to_string().contains("-1"));

        let nd = ContractViolation::NotDescending {
            index: 1,
            lhs: 0.3,
            rhs: 0.7,
        };
        let msg = nd.to_string();
        assert!(msg.contains("hits[1]"));
        assert!(msg.contains("hits[2]"));
    }
}
