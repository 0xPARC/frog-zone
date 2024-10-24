use crate::{custom, internal_server_error};
use core::fmt::Debug;
use rocket::{response::status::Custom, serde::json::Json};
use serde::{de::DeserializeOwned, Serialize};

pub use server::client::*;
pub use server::worker::*;

pub async fn proxy<R: Serialize, S: Debug + DeserializeOwned>(
    server_uri: impl AsRef<str>,
    path: impl AsRef<str>,
    body: R,
) -> Result<Json<S>, Custom<String>> {
    // Create a client
    let client = reqwest::Client::new();

    let json = serde_json::to_string_pretty(&body).unwrap();
    tracing::debug!("Post data: {}", json);

    // Send the request
    let response = client
        .post(format!("{}{}", server_uri.as_ref(), path.as_ref()))
        .json(&body)
        .send()
        .await
        .map_err(internal_server_error)?;

    // Check if the request was successful
    if response.status().is_success() {
        let body = response.json().await.map_err(internal_server_error)?;
        tracing::debug!("Response: {body:?}");
        Ok(Json(body))
    } else {
        let status = response.status();
        let body = response.text().await.map_err(internal_server_error)?;
        tracing::error!("Request failed with status: {status} body: {body}");
        Err(custom(status, body))
    }
}
