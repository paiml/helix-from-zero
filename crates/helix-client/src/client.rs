//! HTTP wrapper for HelixDB QUERY endpoints.

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};
use crate::types::{FilmHit, ListTopFilmsParams};

/// Minimal typed client for a HelixDB instance.
///
/// Constructed against a base URL (default `http://localhost:6969` for
/// `helix push dev`) and reuses one underlying [`reqwest::Client`] across
/// every QUERY call so the connection pool stays warm.
#[derive(Debug, Clone)]
pub struct HelixClient {
    base_url: String,
    http: Client,
}

impl HelixClient {
    /// Build a client pointed at a HelixDB instance.
    ///
    /// Strips a trailing `/` so concatenation with `/query/<name>` is
    /// safe regardless of how the caller spelled the URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        let mut base = base_url.into();
        if base.ends_with('/') {
            base.pop();
        }
        Self {
            base_url: base,
            http: Client::new(),
        }
    }

    /// Base URL the client is talking to (with any trailing slash stripped).
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Generic typed call. POSTs `params` as JSON to `/query/<name>` and
    /// decodes the response as `R`.
    pub async fn query<P, R>(&self, name: &str, params: &P) -> Result<R>
    where
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}/query/{}", self.base_url, name);
        let resp = self.http.post(url).json(params).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Status {
                status: status.as_u16(),
                body,
            });
        }
        Ok(resp.json::<R>().await?)
    }

    /// Call `listTopFilms(genre, limit)` and return typed hits.
    pub async fn list_top_films(&self, genre: &str, limit: i64) -> Result<Vec<FilmHit>> {
        let params = ListTopFilmsParams {
            genre: genre.into(),
            limit,
        };
        self.query("listTopFilms", &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn new_strips_trailing_slash() {
        let client = HelixClient::new("http://localhost:6969/");
        assert_eq!(client.base_url(), "http://localhost:6969");
    }

    #[test]
    fn new_preserves_url_without_trailing_slash() {
        let client = HelixClient::new("http://localhost:6969");
        assert_eq!(client.base_url(), "http://localhost:6969");
    }

    #[test]
    fn new_handles_owned_string() {
        let url = String::from("http://localhost:6969/");
        let client = HelixClient::new(url);
        assert_eq!(client.base_url(), "http://localhost:6969");
    }

    #[test]
    fn client_is_clone() {
        let client = HelixClient::new("http://localhost:6969");
        let cloned = client.clone();
        assert_eq!(cloned.base_url(), "http://localhost:6969");
    }

    #[tokio::test]
    async fn list_top_films_decodes_typed_rows() {
        let mut server = Server::new_async().await;
        let body = serde_json::json!([
            {"id": 1, "title": "A", "content": "x", "source": "Comedy", "score": 0.9},
            {"id": 2, "title": "B", "content": "y", "source": "Comedy", "score": 0.8}
        ]);
        let m = server
            .mock("POST", "/query/listTopFilms")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .create_async()
            .await;

        let client = HelixClient::new(server.url());
        let hits = client.list_top_films("Comedy", 2).await.unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].id, 1);
        assert_eq!(hits[0].score, 0.9);
        assert_eq!(hits[1].title, "B");
        m.assert_async().await;
    }

    #[tokio::test]
    async fn query_returns_status_error_on_5xx() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/query/listTopFilms")
            .with_status(500)
            .with_body("boom")
            .create_async()
            .await;

        let client = HelixClient::new(server.url());
        let err = client.list_top_films("Comedy", 5).await.unwrap_err();
        assert!(err.is_status());
        assert!(err.to_string().contains("500"));
        assert!(err.to_string().contains("boom"));
    }

    #[tokio::test]
    async fn query_returns_status_error_with_empty_body_on_5xx() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/query/listTopFilms")
            .with_status(503)
            .create_async()
            .await;

        let client = HelixClient::new(server.url());
        let err = client.list_top_films("Comedy", 5).await.unwrap_err();
        assert!(err.is_status());
        assert!(err.to_string().contains("503"));
    }

    #[tokio::test]
    async fn query_returns_http_error_on_bad_json() {
        // reqwest folds JSON decode failures into reqwest::Error, so a
        // 200 with non-JSON body lands as Error::Http (decode kind).
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/query/listTopFilms")
            .with_status(200)
            .with_body("not json")
            .create_async()
            .await;

        let client = HelixClient::new(server.url());
        let err = client.list_top_films("Comedy", 5).await.unwrap_err();
        assert!(err.is_http());
    }

    #[tokio::test]
    async fn query_returns_http_error_on_connection_refused() {
        // Bind-and-drop trick: pick an OS-assigned port, drop the listener,
        // then point the client at the now-closed port. reqwest fails fast.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let client = HelixClient::new(format!("http://127.0.0.1:{port}"));
        let err = client.list_top_films("Comedy", 5).await.unwrap_err();
        assert!(err.is_http());
    }
}
