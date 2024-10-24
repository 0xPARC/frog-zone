use core::fmt::Debug;
use phantom::{PhantomBatchedCt, PhantomPackedCt, PhantomPk, PhantomRound1Key, PhantomRound2Key};
use serde::{Deserialize, Serialize};

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
