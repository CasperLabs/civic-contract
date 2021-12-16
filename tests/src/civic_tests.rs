use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256};
use std::collections::BTreeMap;
use test_env::{Sender, TestEnv};

use crate::civic_instance::{CIVICInstance, Meta, TokenId};

const NAME: &str = "CIVIC_KYC";
const SYMBOL: &str = "CKYC";

mod meta {
    use super::{BTreeMap, Meta};
    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "kyc".to_string());
        meta
    }

    pub fn verified_kyc() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("status".to_string(), "verified".to_string());
        meta
    }

    pub fn unverified_kyc() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("status".to_string(), "unverified".to_string());
        meta
    }
}

fn deploy() -> (TestEnv, CIVICInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = CIVICInstance::new(
        &env,
        NAME,
        Sender(owner),
        NAME,
        SYMBOL,
        meta::contract_meta(),
        owner,
    );
    (env, token, owner)
}

#[test]
fn test_deploy() {
    let (_, token, owner) = deploy();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.meta(), meta::contract_meta());
    assert_eq!(token.total_supply(), U256::zero());
    assert!(token.is_admin(owner));
}

#[test]
fn test_add_gatekeeper() {
    let (env, token, owner) = deploy();
    let user = env.next_user();

    token.grant_gatekeeper(Sender(owner), user);
    assert!(token.is_gatekeeper(user));
}

#[test]
fn test_revoke_gatekeeper() {
    let (env, token, owner) = deploy();
    let user = env.next_user();

    token.grant_gatekeeper(Sender(owner), user);
    assert!(token.is_gatekeeper(user));

    token.revoke_gatekeeper(Sender(owner), user);
    assert!(!token.is_gatekeeper(user));
}

#[test]
fn test_mint_from_gatekeeper() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_id = TokenId::from("custom_token_id");
    let token_meta = meta::unverified_kyc();

    token.grant_gatekeeper(Sender(owner), ali);

    token.mint(Sender(ali), bob, Some(token_id.clone()), token_meta.clone());

    let user_token_meta = token.token_meta(token_id.clone());
    assert_eq!(user_token_meta.unwrap(), token_meta);

    let first_user_token = token.get_token_by_index(Key::Account(bob), U256::zero());
    assert_eq!(first_user_token, Some(token_id));
}

#[test]
#[should_panic]
fn test_mint_from_non_gatekeeper() {
    let (env, token, _) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_id = TokenId::from("custom_token_id");
    let token_meta = meta::unverified_kyc();

    token.mint(Sender(ali), bob, Some(token_id), token_meta);
}

#[test]
fn test_transfer_from_admin() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();

    token.mint(Sender(owner), ali, None, meta::unverified_kyc());
    token.mint(Sender(owner), ali, None, meta::verified_kyc());
    let first_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(0));
    let second_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(1));

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(
        token.owner_of(first_ali_token.clone().unwrap()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(second_ali_token.unwrap()).unwrap(),
        Key::Account(ali)
    );
    token.grant_admin(Sender(owner), ali);
    token.transfer_from(Sender(ali), ali, bob, vec![first_ali_token.unwrap()]);
    let new_first_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(0));
    let new_second_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(1));
    let new_first_bob_token = token.get_token_by_index(Key::Account(bob), U256::from(0));
    let new_second_bob_token = token.get_token_by_index(Key::Account(bob), U256::from(1));
    println!("{:?}", new_first_ali_token);
    println!("{:?}", new_second_ali_token);
    println!("{:?}", new_first_bob_token);
    println!("{:?}", new_second_bob_token);
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(bob)), U256::from(1));
    assert_eq!(
        token.owner_of(new_first_ali_token.unwrap()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(new_first_bob_token.unwrap()).unwrap(),
        Key::Account(bob)
    );
    assert_eq!(new_second_ali_token, None);
    assert_eq!(new_second_bob_token, None);
}

#[test]
#[should_panic]
fn test_transfer_from_non_admin() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();

    token.mint(Sender(owner), ali, None, meta::unverified_kyc());
    token.mint(Sender(owner), ali, None, meta::verified_kyc());
    let first_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(0));
    let second_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(1));

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(
        token.owner_of(first_ali_token.clone().unwrap()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(second_ali_token.unwrap()).unwrap(),
        Key::Account(ali)
    );
    token.transfer_from(Sender(ali), ali, bob, vec![first_ali_token.unwrap()]);
}

#[test]
fn test_token_meta() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("custom_token_id");
    let token_meta = meta::unverified_kyc();

    token.mint(
        Sender(owner),
        user,
        Some(token_id.clone()),
        token_meta.clone(),
    );

    let user_token_meta = token.token_meta(token_id.clone());
    assert_eq!(user_token_meta.unwrap(), token_meta);

    let first_user_token = token.get_token_by_index(Key::Account(user), U256::zero());
    assert_eq!(first_user_token, Some(token_id));
}

#[test]
fn test_token_metadata_set_from_gatekeeper() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_id = TokenId::from("123456");

    token.mint(
        Sender(owner),
        bob,
        Some(token_id.clone()),
        meta::unverified_kyc(),
    );
    token.grant_gatekeeper(Sender(owner), ali);
    token.set_token_meta(Sender(ali), token_id.clone(), meta::verified_kyc());
    assert_eq!(token.token_meta(token_id).unwrap(), meta::verified_kyc());
}

#[test]
#[should_panic]
fn test_token_metadata_set_from_non_gatekeeper() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_id = TokenId::from("123456");

    token.mint(
        Sender(owner),
        bob,
        Some(token_id.clone()),
        meta::unverified_kyc(),
    );
    token.set_token_meta(Sender(ali), token_id, meta::verified_kyc());
}

#[test]
fn test_token_metadata_update_from_gatekeeper() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_id = TokenId::from("123456");

    token.mint(
        Sender(owner),
        bob,
        Some(token_id.clone()),
        meta::unverified_kyc(),
    );
    token.grant_gatekeeper(Sender(owner), ali);
    token.set_token_meta(Sender(ali), token_id.clone(), meta::verified_kyc());
    token.update_token_meta(
        Sender(ali),
        token_id.clone(),
        String::from("expiry"),
        String::from("5555555"),
    );
    let mut expected_result = meta::verified_kyc();
    expected_result.insert(String::from("expiry"), String::from("5555555"));
    assert_eq!(token.token_meta(token_id).unwrap(), expected_result);
}

#[test]
#[should_panic]
fn test_token_metadata_update_from_non_gatekeeper() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_id = TokenId::from("123456");

    token.mint(
        Sender(owner),
        bob,
        Some(token_id.clone()),
        meta::unverified_kyc(),
    );
    token.grant_gatekeeper(Sender(owner), ali);
    token.set_token_meta(Sender(ali), token_id.clone(), meta::verified_kyc());
    token.revoke_gatekeeper(Sender(owner), ali);
    token.update_token_meta(
        Sender(ali),
        token_id,
        String::from("expiry"),
        String::from("5555555"),
    );
}
