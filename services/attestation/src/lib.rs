pub mod types;

use binding_macro::{cycles, service};
use protocol::traits::{ExecutorParams, ServiceResponse, ServiceSDK, StoreMap};
use protocol::types::{Hash, ServiceContext};

use crate::types::{
    AttestationInfoPayload, AttestationInfoResponse, QueryAttestationPayload,
    QueryAttestationResponse,
};

const ATTESTATION_INFO_KEY: &str = "attestation_info_key";

pub trait AttestationInterface {
    fn inner_attest_info(
        &mut self,
        ctx: &ServiceContext,
        payload: AttestationInfoPayload,
    ) -> ServiceResponse<AttestationInfoResponse>;

    fn inner_query(
        &self,
        ctx: &ServiceContext,
        payload: QueryAttestationPayload,
    ) -> ServiceResponse<QueryAttestationResponse>;
}

pub struct AttestationService<SDK> {
    _sdk: SDK,
    attestation_info: Box<dyn StoreMap<Hash, String>>,
}

#[service]
impl<SDK: ServiceSDK> AttestationService<SDK> {
    pub fn new(mut _sdk: SDK) -> Self {
        let attestation_info = _sdk.alloc_or_recover_map(ATTESTATION_INFO_KEY);
        AttestationService {
            _sdk,
            attestation_info,
        }
    }

    #[cycles(210_00)]
    #[write]
    fn attest_info(
        &mut self,
        ctx: ServiceContext,
        payload: AttestationInfoPayload,
    ) -> ServiceResponse<AttestationInfoResponse> {
        if let Some(hash) = ctx.get_tx_hash() {
            self.attestation_info.insert(hash.clone(), payload.info);
            ServiceResponse::from_succeed(AttestationInfoResponse { hash })
        } else {
            ServiceResponse::from_error(101, "Missing tx hash".to_owned())
        }
    }

    #[cycles(210_00)]
    #[read]
    fn query(
        &self,
        ctx: ServiceContext,
        payload: QueryAttestationPayload,
    ) -> ServiceResponse<QueryAttestationResponse> {
        if let Some(info) = self.attestation_info.get(&payload.hash) {
            ServiceResponse::from_succeed(QueryAttestationResponse { info })
        } else {
            ServiceResponse::from_error(102, "Can not get attestation info".to_owned())
        }
    }
}
