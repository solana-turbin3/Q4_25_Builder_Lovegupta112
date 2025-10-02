# Turbin3 Rust Enrollment Prerequisites

It demonstrates how to create wallets, claim SOL airdrops, transfer tokens, empty balances, and finally interact with the Turbin3 enrollment dApp on Solana Devnet.

## Overview

The codebase covers the following:

1. **Wallet Creation**
   - Generate a new Solana keypair
   - Save it to a local JSON file
   - Convert between Phantom (Base58) format and Solana wallet JSON format

2. **Airdrop Requests**
   - Connect to Solana Devnet via RPC
   - Request 2 SOL airdrop into the generated wallet

3. **SOL Transfers**
   - Send SOL from your devnet wallet to your Turbin3 wallet
   - Verify signatures from your keypair
   - Drain your wallet balance into your Turbin3 wallet (accounting for fees)

4. **Program Interaction**
   - Interact with the Turbin3 Enrollment dApp deployed on Solana Devnet  
   - Use PDAs (Program Derived Addresses) and custom instructions  
   - Submit proof of Rust prerequisite completion via the `submit_rs` instruction

5. **NFT Minting**
   - Upon successful interaction, mint an NFT that proves your completion of the Rust prerequisites
