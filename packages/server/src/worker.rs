use crate::{custom, internal_server_error, zone::ZoneDiff};
use phantom::{
    PhantomBatchedCt, PhantomBsKey, PhantomCt, PhantomPackedCt, PhantomPk, PhantomRpKey,
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::{json::Json, Deserialize, Serialize},
};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, ops::Deref};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitRequest {
    pub zone_width: u8,
    pub zone_height: u8,
    pub zone_cts: Vec<PhantomCt>,
    pub pk: PhantomPk,
    pub bs_key: PhantomBsKey,
    pub rp_key: PhantomRpKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestWithDiff<R> {
    pub request: R,
    pub diff: ZoneDiff,
}

impl<R> Deref for RequestWithDiff<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

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

pub async fn request<R: Serialize, S: Debug + DeserializeOwned>(
    worker_uri: impl AsRef<str>,
    path: impl AsRef<str>,
    body: R,
) -> Result<Json<S>, Custom<String>> {
    // Create a client
    let client = reqwest::Client::new();

    let json = serde_json::to_string_pretty(&body).unwrap();
    tracing::debug!("Post data: {}", json);

    // Send the request
    let response = client
        .post(format!("{}{}", worker_uri.as_ref(), path.as_ref()))
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
        Err(custom(Status::from_code(status.as_u16()).unwrap(), body))
    }
}
