use crate::{custom, internal_server_error};
use core::fmt::Debug;
use phantom::{PhantomBatchedCt, PhantomPackedCt, PhantomPk, PhantomRound1Key, PhantomRound2Key};
use rocket::{response::status::Custom, serde::json::Json};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCellsRequest {
    pub player_id: usize,
    pub coords: PhantomBatchedCt, // Vec<EncryptedCoord>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCellsResponse {
    pub cell_data: PhantomPackedCt, // Vec<CellEncryptedData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFiveCellsRequest {
    pub player_id: usize,
    pub coords: PhantomBatchedCt, // [EncryptedCoord; 5]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFiveCellsResponse {
    pub cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCrossCellsRequest {
    pub player_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCrossCellsResponse {
    pub cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVerticalCellsRequest {
    pub player_id: usize,
    pub coord: PhantomBatchedCt, // EncryptedCoord
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVerticalCellsResponse {
    pub cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHorizontalCellsRequest {
    pub player_id: usize,
    pub coord: PhantomBatchedCt, // EncryptedCoord
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHorizontalCellsResponse {
    pub cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlayerRequest {
    pub player_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlayerResponse {
    pub player_data: PhantomPackedCt, // PlayerEncryptedData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveRequest {
    pub player_id: usize,
    pub direction: PhantomBatchedCt, // Encrypted<Direction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveResponse {
    pub my_new_coords: Option<PhantomPackedCt>, // EncryptedCoord
    pub rate_limited: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitRound1KeyRequest {
    pub player_id: usize,
    pub key: PhantomRound1Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitRound1KeyResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPkRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPkResponse {
    pub pk: PhantomPk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitRound2KeyRequest {
    pub player_id: usize,
    pub key: PhantomRound2Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitRound2KeyResponse {}

pub async fn proxy<R: Serialize, S: Debug + DeserializeOwned>(
    path: impl AsRef<str>,
    body: R,
) -> Result<Json<S>, Custom<String>> {
    // Create a client
    let client = reqwest::Client::new();

    let json = serde_json::to_string_pretty(&body).unwrap();
    tracing::debug!("Post data: {}", json);

    // Send the request
    let server_uri = "http://localhost:8000";
    let response = client
        .post(format!("{server_uri}{}", path.as_ref()))
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
