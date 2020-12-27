mod types;

use attestation::AttestationInterface;
use binding_macro::{cycles, service};
use protocol::traits::{ExecutorParams, ServiceResponse, ServiceSDK, StoreMap};
use protocol::types::{Hash, ServiceContext};
use rand::{thread_rng, Rng};

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
impl<SDK: ServiceSDK> LotterydrawService<SDK> {}

fn lottery_guys<T: Clone>(input: Vec<T>, num: usize) -> Vec<T> {
    let mut rng = thread_rng();
    let mut index_set = (0..input.len()).collect::<Vec<_>>();

    let mut ret = Vec::new();
    for _i in 0..num {
        let idx = rng.gen_range(0, index_set.len());
        ret.push(input.get(index_set.remove(idx)).cloned().unwrap());
    }
    ret
}
