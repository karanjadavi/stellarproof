#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test_verify_kyc_valid() {
    let env = Env::default();
    let contract_id = env.register(ZKVerifierContract, ());
    let client = ZKVerifierContractClient::new(&env, &contract_id);

    // Valid proof - age 25, Kenya (254)
    let result = client.verify_kyc(&18, &254, &true);
    assert_eq!(result, true);
}

#[test]
fn test_verify_kyc_invalid() {
    let env = Env::default();
    let contract_id = env.register(ZKVerifierContract, ());
    let client = ZKVerifierContractClient::new(&env, &contract_id);

    // Invalid proof
    let result = client.verify_kyc(&18, &254, &false);
    assert_eq!(result, false);
}
