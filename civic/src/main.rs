#![no_main]

use cep47::{self, endpoint, get_caller, CEP47Contract, CasperCEP47Contract, Meta, TokenId};
use contract::{
    contract_api::{
        runtime::{get_key, get_named_arg, put_key, remove_key, revert},
        storage::new_uref,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{ApiError, CLType, Key, Parameter};

#[no_mangle]
pub extern "C" fn add_gateway() {
    let gateway = get_named_arg::<Key>("gateway");
    let gateway_key = gateway_key(&gateway);
    match get_key(&gateway_key) {
        None => {
            let key = new_uref(gateway).into();
            put_key(&gateway_key, key);
        }
        Some(_) => {
            panic!("Trying to add an existing gateway");
        }
    }
}

#[no_mangle]
pub extern "C" fn revoke_gateway() {
    let gateway = get_named_arg::<Key>("gateway");
    let gateway_key = gateway_key(&gateway);
    match get_key(&gateway_key) {
        None => {
            panic!("Trying to revoke non-existing gateway");
        }
        Some(_) => {
            remove_key(&gateway_key);
        }
    }
}

#[no_mangle]
pub extern "C" fn mint_one() {
    let sender = get_caller();
    if !is_gateway(&sender) {
        revert(ApiError::PermissionDenied);
    }
    let recipient = get_named_arg::<Key>("recipient");
    let token_id = get_named_arg::<Option<TokenId>>("token_id");
    let token_meta = get_named_arg::<Meta>("token_meta");
    let mut contract = CasperCEP47Contract::new();
    contract
        .mint_one(&recipient, token_id, token_meta)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn mint_many() {
    let sender = get_caller();
    if !is_gateway(&sender) {
        revert(ApiError::PermissionDenied);
    }
    let recipient = get_named_arg::<Key>("recipient");
    let token_ids = get_named_arg::<Option<Vec<TokenId>>>("token_ids");
    let token_metas = get_named_arg::<Vec<Meta>>("token_metas");
    let mut contract = CasperCEP47Contract::new();
    contract
        .mint_many(&recipient, token_ids, token_metas)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn mint_copies() {
    let sender = get_caller();
    if !is_gateway(&sender) {
        revert(ApiError::PermissionDenied);
    }
    let recipient = get_named_arg::<Key>("recipient");
    let token_ids = get_named_arg::<Option<Vec<TokenId>>>("token_ids");
    let token_meta = get_named_arg::<Meta>("token_meta");
    let count = get_named_arg::<u32>("count");
    let mut contract = CasperCEP47Contract::new();
    contract
        .mint_copies(&recipient, token_ids, token_meta, count)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn_one() {
    let sender = get_caller();
    if !is_gateway(&sender) {
        revert(ApiError::PermissionDenied);
    }
    let owner = get_named_arg::<Key>("owner");
    let token_id = get_named_arg::<TokenId>("token_id");
    let mut contract = CasperCEP47Contract::new();
    contract.burn_one(&owner, token_id);
}

#[no_mangle]
pub extern "C" fn burn_many() {
    let sender = get_caller();
    if !is_gateway(&sender) {
        revert(ApiError::PermissionDenied);
    }
    let owner = get_named_arg::<Key>("owner");
    let token_ids = get_named_arg::<Vec<TokenId>>("token_ids");
    let mut contract = CasperCEP47Contract::new();
    contract.burn_many(&owner, token_ids);
}

#[no_mangle]
pub extern "C" fn update_token_metadata() {
    let sender = get_caller();
    if !is_gateway(&sender) {
        revert(ApiError::PermissionDenied);
    }
    let token_id = get_named_arg::<TokenId>("token_id");
    let meta = get_named_arg::<Meta>("token_meta");
    let mut contract = CasperCEP47Contract::new();
    let res = contract.update_token_metadata(token_id, meta);
    res.unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) =
        contract::contract_api::storage::create_contract_package_at_hash();
    let mut entry_points = cep47::get_entrypoints(Some(contract_package_hash));

    entry_points.add_entry_point(endpoint(
        "add_gateway",
        vec![Parameter::new("gateway", CLType::Key)],
        CLType::Unit,
        Some("deployer"),
    ));

    entry_points.add_entry_point(endpoint(
        "revoke_gatekeepr",
        vec![Parameter::new("gateway", CLType::Key)],
        CLType::Unit,
        Some("deployer"),
    ));

    cep47::deploy(
        get_named_arg::<String>("token_name"),
        get_named_arg::<String>("token_symbol"),
        get_named_arg::<cep47::Meta>("token_uri"),
        entry_points,
        contract_package_hash,
        false,
    );
}

fn gateway_key(account: &Key) -> String {
    format!("gateway_{}", account)
}

fn is_gateway(account: &Key) -> bool {
    let gateway_key = gateway_key(&account);
    match get_key(&gateway_key) {
        None => false,
        Some(_) => true,
    }
}
