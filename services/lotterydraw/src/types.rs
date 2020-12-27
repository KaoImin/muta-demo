use muta_codec_derive::RlpFixedCodec;
use protocol::fixed_codec::{FixedCodec, FixedCodecError};
use protocol::types::{Bytes, Hash};
use protocol::ProtocolResult;
use serde::{Deserialize, Serialize};

type JsonString = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LotterydrawPayload {
    pub tx_hashes: Vec<Hash>,
    pub lottery_num: usize,
}

#[derive(Default, RlpFixedCodec, Serialize, Deserialize, Clone, Debug)]
pub struct LotterydrawResponse {
    pub lottery_guys: Vec<String>,
}
