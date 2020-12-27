mod types;

use attestation::AttestationInterface;
use binding_macro::{cycles, service};
use protocol::traits::{ExecutorParams, ServiceResponse, ServiceSDK, StoreMap};
use protocol::types::{Hash, ServiceContext};

use crate::types::{LotterydrawPayload, LotterydrawResponse};

pub trait LotterydrawInterface {
    fn inner_lotterdraw(
        &mut self,
        ctx: &ServiceContext,
        payload: LotterydrawPayload,
    ) -> ServiceResponse<LotterydrawResponse>;
}

pub struct LotterydrawService<SDK> {
    sdk: SDK,
    lottery_guys: Box<dyn StoreMap<Hash, LotterydrawResponse>>,
    attestation: Box<dyn AttestationInterface>,
}

#[service] 
impl<SDK: ServiceSDK> LotterydrawService<SDK> {

}
