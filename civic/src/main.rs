#![no_main]

use cep47::{
    endpoint, get_key,
    logic::{CEP47Contract, TokenId, URI},
    set_key, CasperCEP47Contract,
};
use contract::{
    contract_api::runtime::{self, revert},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{ApiError, CLType, PublicKey, U256};

#[no_mangle]
pub extern "C" fn transfer_token() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let sender: PublicKey = runtime::get_named_arg("sender");
            let recipient: PublicKey = runtime::get_named_arg("recipient");
            let token_id: TokenId = runtime::get_named_arg("token_id");
            let mut contract = CasperCEP47Contract::new();
            let res = contract.transfer_token(sender, recipient, token_id);
            res.unwrap_or_revert();
        }
    }
}

#[no_mangle]
pub extern "C" fn transfer_many_tokens() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let sender: PublicKey = runtime::get_named_arg("sender");
            let recipient: PublicKey = runtime::get_named_arg("recipient");
            let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
            let mut contract = CasperCEP47Contract::new();
            let res = contract.transfer_many_tokens(sender, recipient, token_ids);
            res.unwrap_or_revert();
        }
    }
}

#[no_mangle]
pub extern "C" fn transfer_all_tokens() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let sender: PublicKey = runtime::get_named_arg("sender");
            let recipient: PublicKey = runtime::get_named_arg("recipient");
            let mut contract = CasperCEP47Contract::new();
            contract.transfer_all_tokens(sender, recipient);
        }
    }
}

#[no_mangle]
pub extern "C" fn mint_one() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let recipient: PublicKey = runtime::get_named_arg("recipient");
            let token_uri: URI = runtime::get_named_arg("token_uri");
            let mut contract = CasperCEP47Contract::new();
            contract.mint_one(recipient, token_uri);
        }
    }
}

#[no_mangle]
pub extern "C" fn mint_many() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let recipient: PublicKey = runtime::get_named_arg("recipient");
            let token_uris: Vec<URI> = runtime::get_named_arg("token_uris");
            let mut contract = CasperCEP47Contract::new();
            contract.mint_many(recipient, token_uris);
        }
    }
}

#[no_mangle]
pub extern "C" fn mint_copies() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let recipient: PublicKey = runtime::get_named_arg("recipient");
            let token_uri: URI = runtime::get_named_arg("token_uri");
            let count: U256 = runtime::get_named_arg("count");
            let mut contract = CasperCEP47Contract::new();
            contract.mint_copies(recipient, token_uri, count);
        }
    }
}

#[no_mangle]
pub extern "C" fn burn_many() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let owner: PublicKey = runtime::get_named_arg("owner");
            let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
            let mut contract = CasperCEP47Contract::new();
            contract.burn_many(owner, token_ids);
        }
    }
}

#[no_mangle]
pub extern "C" fn burn_one() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::PermissionDenied),
        false => {
            let owner: PublicKey = runtime::get_named_arg("owner");
            let token_id: TokenId = runtime::get_named_arg("token_id");
            let mut contract = CasperCEP47Contract::new();
            contract.burn_one(owner, token_id);
        }
    }
}

#[no_mangle]
pub extern "C" fn pause() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => revert(ApiError::None),
        false => {
            set_key("paused", true);
        }
    }
}

#[no_mangle]
pub extern "C" fn unpause() {
    let paused = get_key::<bool>("paused").unwrap_or_revert();
    match paused {
        true => {
            set_key("paused", false);
        }
        false => revert(ApiError::None),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) =
        contract::contract_api::storage::create_contract_package_at_hash();
    let mut entry_points = cep47::get_entrypoints(Some(contract_package_hash));

    entry_points.add_entry_point(endpoint(
        "pause",
        vec![],
        CLType::Unit,
        Some("deployer_access"),
    ));
    entry_points.add_entry_point(endpoint(
        "unpause",
        vec![],
        CLType::Unit,
        Some("deployer_access"),
    ));

    cep47::deploy(
        &contract::contract_api::runtime::get_named_arg::<String>("token_name"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_symbol"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_uri"),
        entry_points,
        contract_package_hash,
    );
}
