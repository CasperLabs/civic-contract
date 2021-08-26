#![no_main]
#![no_std]
#[macro_use]
extern crate alloc;

use alloc::{
    boxed::Box,
    collections::BTreeSet,
    string::{String, ToString},
    vec::Vec,
};
use cep47::{
    contract_utils::{AdminControl, ContractContext, OnChainContractStorage},
    Meta, TokenId, CEP47,
};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    contracts::NamedKeys, runtime_args, ApiError, CLType, CLTyped, CLValue, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};

mod gatekeeper_control;
use gatekeeper_control::GateKeeperControl;

#[derive(Default)]
struct GatewayToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for GatewayToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP47<OnChainContractStorage> for GatewayToken {}
impl AdminControl<OnChainContractStorage> for GatewayToken {}
impl GateKeeperControl<OnChainContractStorage> for GatewayToken {}
impl GatewayToken {
    fn constructor(&mut self, name: String, symbol: String, meta: Meta) {
        CEP47::init(self, name, symbol, meta);
        AdminControl::init(self);
        GateKeeperControl::init(self);
    }
}

#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>("name");
    let symbol = runtime::get_named_arg::<String>("symbol");
    let meta = runtime::get_named_arg::<Meta>("meta");
    GatewayToken::default().constructor(name, symbol, meta);
}

#[no_mangle]
fn name() {
    let ret = GatewayToken::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret = GatewayToken::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn meta() {
    let ret = GatewayToken::default().meta();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret = GatewayToken::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let ret = GatewayToken::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_token_by_index() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let index = runtime::get_named_arg::<U256>("index");
    let ret = GatewayToken::default().get_token_by_index(owner, index);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn owner_of() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = GatewayToken::default().owner_of(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = GatewayToken::default().token_meta(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn update_token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    GatewayToken::default()
        .set_token_meta(token_id, token_meta)
        .unwrap_or_revert();
}

#[no_mangle]
fn mint() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_id = runtime::get_named_arg::<Option<TokenId>>("token_id");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    if !GatewayToken::default().is_gatekeeper() {
        runtime::revert(ApiError::User(20));
    }
    GatewayToken::default()
        .mint(recipient, token_id.map(|x| vec![x]), vec![token_meta])
        .unwrap_or_revert();
}

#[no_mangle]
fn burn() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    if !GatewayToken::default().is_gatekeeper() {
        runtime::revert(ApiError::User(20));
    }
    GatewayToken::default()
        .burn(owner, vec![token_id])
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    if !GatewayToken::default().is_gatekeeper() {
        runtime::revert(ApiError::User(20));
    }
    GatewayToken::default()
        .transfer(recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer_from() {
    let sender = runtime::get_named_arg::<Key>("sender");
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    if !GatewayToken::default().is_gatekeeper() {
        runtime::revert(ApiError::User(20));
    }
    GatewayToken::default()
        .transfer_from(sender, recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn approve() {
    let spender = runtime::get_named_arg::<Key>("spender");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    if !GatewayToken::default().is_gatekeeper() {
        runtime::revert(ApiError::User(20));
    }
    GatewayToken::default()
        .approve(spender, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn get_approved() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = GatewayToken::default().get_approved(owner, token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn add_gatekeeper() {
    let gatekeeper = runtime::get_named_arg::<Key>("gatekeeper");
    GatewayToken::default().assert_caller_is_admin();
    GatewayToken::default().add_gatekeeper(gatekeeper);
}

#[no_mangle]
fn revoke_gatekeeper() {
    let gatekeeper = runtime::get_named_arg::<Key>("gatekeeper");
    GatewayToken::default().assert_caller_is_admin();
    GatewayToken::default().revoke_gatekeeper(gatekeeper);
}

#[no_mangle]
pub extern "C" fn call() {
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let mut named_keys = NamedKeys::new();
    let contract_package_hash_wrapped = storage::new_uref(package_hash).into();
    named_keys.insert(
        "contract_package_hash".to_string(),
        contract_package_hash_wrapped,
    );
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), named_keys);

    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let meta: Meta = runtime::get_named_arg("meta");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "symbol" => symbol,
        "meta" => meta
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        contract_package_hash_wrapped,
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("meta", Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "meta",
        vec![],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner_of",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_meta",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_token_meta",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("token_meta", Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new(
                "token_ids",
                CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type())))),
            ),
            Parameter::new("token_metas", CLType::List(Box::new(Meta::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_copies",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new(
                "token_ids",
                CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type())))),
            ),
            Parameter::new("token_meta", Meta::cl_type()),
            Parameter::new("count", CLType::U32),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("sender", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_approved",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token_by_index",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("index", U256::cl_type()),
        ],
        CLType::Option(Box::new(TokenId::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_getkeeper",
        vec![Parameter::new("gatekeeper", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token_by_index",
        vec![Parameter::new("gatekeeper", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
