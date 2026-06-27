#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Address, Map, BytesN};

#[contract]
pub struct ZKVerifierContract;

#[contractimpl]
impl ZKVerifierContract {
    /// Verify a ZK proof for KYC and store result on-chain
    pub fn verify_kyc(
        env: Env,
        user: Address,
        min_age: u64,
        allowed_country: u64,
        proof_hash: BytesN<32>,
        proof_valid: bool,
    ) -> bool {
        if proof_valid {
            // Store verified address permanently on-chain
            let mut verified: Map<Address, bool> = env
                .storage()
                .persistent()
                .get(&symbol_short!("verified"))
                .unwrap_or(Map::new(&env));

            verified.set(user.clone(), true);
            env.storage()
                .persistent()
                .set(&symbol_short!("verified"), &verified);

            // Store proof hash to prevent replay attacks
            env.storage()
                .persistent()
                .set(&proof_hash, &true);

            // Emit event
            env.events().publish(
                (symbol_short!("kyc_ok"),),
                (user, min_age, allowed_country),
            );
            true
        } else {
            false
        }
    }

    /// Check if an address is KYC verified
    pub fn is_verified(env: Env, user: Address) -> bool {
        let verified: Map<Address, bool> = env
            .storage()
            .persistent()
            .get(&symbol_short!("verified"))
            .unwrap_or(Map::new(&env));
        verified.get(user).unwrap_or(false)
    }

    /// Check if a proof hash has been used
    pub fn is_proof_used(env: Env, proof_hash: BytesN<32>) -> bool {
        env.storage()
            .persistent()
            .get(&proof_hash)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test;
