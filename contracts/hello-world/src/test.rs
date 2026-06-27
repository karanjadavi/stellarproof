#![cfg(test)]
use super::*;
use soroban_sdk::{Env, BytesN};
use soroban_sdk::testutils::Address as _;

#[test]
fn test_verify_kyc_valid() {
    let env = Env::default();
    let contract_id = env.register(ZKVerifierContract, ());
    let client = ZKVerifierContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let proof_hash = BytesN::from_array(&env, &[1u8; 32]);

    let result = client.verify_kyc(&user, &18, &254, &proof_hash, &true);
    assert_eq!(result, true);

    let is_verified = client.is_verified(&user);
    assert_eq!(is_verified, true);
}

#[test]
fn test_verify_kyc_invalid() {
    let env = Env::default();
    let contract_id = env.register(ZKVerifierContract, ());
    let client = ZKVerifierContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let proof_hash = BytesN::from_array(&env, &[2u8; 32]);

    let result = client.verify_kyc(&user, &18, &254, &proof_hash, &false);
    assert_eq!(result, false);

    let is_verified = client.is_verified(&user);
    assert_eq!(is_verified, false);
}

#[test]
fn test_proof_hash_stored() {
    let env = Env::default();
    let contract_id = env.register(ZKVerifierContract, ());
    let client = ZKVerifierContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let proof_hash = BytesN::from_array(&env, &[3u8; 32]);

    client.verify_kyc(&user, &18, &254, &proof_hash, &true);

    let is_used = client.is_proof_used(&proof_hash);
    assert_eq!(is_used, true);
}
