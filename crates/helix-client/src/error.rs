//! Error type for the helix-client crate.

use thiserror::Error;

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can happen when calling a HelixDB HTTP endpoint.
///
/// reqwest folds transport failures and JSON decode failures into the
/// same [`reqwest::Error`] type, so the [`Error::Http`] variant carries
/// both. [`Error::Status`] is the explicit non-2xx case where the helix
/// instance answered the request but rejected it.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP layer error — connection failure, transport issue, or JSON decode failure.
    #[error("HTTP error talking to helix instance: {0}")]
    Http(#[from] reqwest::Error),

    /// HelixDB returned a non-2xx status; carries the status code and body text.
    #[error("helix endpoint returned status {status}: {body}")]
    Status {
        /// HTTP status code returned by the helix instance.
        status: u16,
        /// Body text returned by the helix instance.
        body: String,
    },
}

impl Error {
    /// True when this is the [`Error::Http`] variant.
    pub fn is_http(&self) -> bool {
        matches!(self, Self::Http(_))
    }

    /// True when this is the [`Error::Status`] variant.
    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_status(code: u16, body: &str) -> Error {
        Error::Status {
            status: code,
            body: body.into(),
        }
    }

    #[test]
    fn is_http_true_for_http_variant() {
        // Connection-refused trick: bind, drop, dial — produces a real
        // reqwest::Error wrapped as Error::Http with no external service.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let err: Error = rt.block_on(async {
            reqwest::Client::new()
                .get(format!("http://127.0.0.1:{port}"))
                .send()
                .await
                .unwrap_err()
                .into()
        });
        assert!(err.is_http());
        assert!(!err.is_status());
    }

    #[test]
    fn is_status_true_for_status_variant() {
        let err = make_status(500, "boom");
        assert!(err.is_status());
        assert!(!err.is_http());
    }

    #[test]
    fn display_status_shows_code_and_body() {
        let err = make_status(404, "missing");
        let s = err.to_string();
        assert!(s.contains("status 404"));
        assert!(s.contains("missing"));
    }
}
