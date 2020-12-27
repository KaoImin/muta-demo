use protocol::types::Hash;

use serde::{Deserialize, Serialize};

type JsonString = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttestationInfoPayload {
    pub info: JsonString,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct AttestationInfoResponse {
    pub hash: Hash,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct QueryAttestationPayload {
    pub hash: Hash,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct QueryAttestationResponse {
    pub info: JsonString,
}
