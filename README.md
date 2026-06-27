# StellarProof

A privacy-preserving KYC verification system built on Stellar using Zero Knowledge proofs.

## What it does

Users prove they meet conditions like age and nationality using Noir ZK proofs verified inside a Soroban smart contract without revealing private data. Verified addresses are stored permanently on-chain.

## Tech Stack

- Noir: ZK circuit for private proof generation
- Soroban/Rust: Smart contract on Stellar
- Stellar Testnet: Blockchain deployment
- Next.js + TypeScript: Frontend

## Contract Functions

- verify_kyc: Verifies ZK proof and stores address on-chain
- is_verified: Checks if an address is KYC verified
- is_proof_used: Checks if a proof hash has been used

## Contract Details

- Network: Stellar Testnet
- Contract ID: CAB232Q6K6WIHILWAAHW4K5DQQ4Q3ZRGOHSMRZSBTCXKDAHTOIVBDVCV
- Explorer: https://stellar.expert/explorer/testnet/contract/CAB232Q6K6WIHILWAAHW4K5DQQ4Q3ZRGOHSMRZSBTCXKDAHTOIVBDVCV

## Built by

David Auja (@karanjadavi) - Stellar Hacks: Real-World ZK Hackathon 2026
