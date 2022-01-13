use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, Key};
use cep47::contract_utils::{ContractContext, ContractStorage, Dict};

const GATEKEEPERS_DICT: &str = "gatekeepers";
pub trait GateKeeperControl<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {
        GateKeepers::init();
    }

    fn revoke_gatekeeper(&mut self, address: Key) {
        GateKeepers::instance().revoke_gatekeeper(&address);
    }

    fn add_gatekeeper(&mut self, address: Key) {
        GateKeepers::instance().add_gatekeeper(&address);
    }

    fn is_gatekeeper(&self) -> bool {
        let caller = self.get_caller();
        GateKeepers::instance().is_gatekeeper(&caller)
    }

    fn assert_caller_is_gatekeeper(&self) {
        if !self.is_gatekeeper() {
            runtime::revert(ApiError::User(20));
        }
    }
}

struct GateKeepers {
    dict: Dict,
}

impl GateKeepers {
    pub fn instance() -> GateKeepers {
        GateKeepers {
            dict: Dict::instance(GATEKEEPERS_DICT),
        }
    }
    pub fn init() {
        storage::new_dictionary(GATEKEEPERS_DICT).unwrap_or_revert();
    }

    pub fn is_gatekeeper(&self, key: &Key) -> bool {
        self.dict.get_by_key::<()>(key).is_some()
    }

    pub fn add_gatekeeper(&self, key: &Key) {
        self.dict.set_by_key(key, ());
    }

    pub fn revoke_gatekeeper(&self, key: &Key) {
        self.dict.remove_by_key::<()>(key);
    }
}
