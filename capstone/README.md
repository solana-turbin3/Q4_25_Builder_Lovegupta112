# YieldPay - Stake, Earn, and Pay with Yield

A decentralized staking and payment protocol built on Solana using Anchor framework. Stake your tokens, earn yield, and pay merchants using your accumulated rewards.


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?logo=solana)](https://explorer.solana.com/address/7ssJMQw9tFamJcsdxuaEwM6iKF7LS3e2ypNNFKRcLHjA?cluster=devnet)



## 🌟 What is YieldPay?

YieldPay is an innovative DeFi protocol that combines staking, yield generation, and payment functionality. Users can stake whitelisted tokens to earn yield over time, then use that yield as a payment method for merchant services. Unlike traditional payment systems, YieldPay enables:

* **Passive Income Generation**: Earn yield on staked tokens automatically
* **Payment with Rewards**: Pay merchants using accumulated yield without touching principal
* **Multi-Token Support**: Stake various whitelisted tokens to generate universal yield tokens
* **Merchant Services**: Businesses can receive payments in yield tokens and onboard easily
* **Capital Efficiency**: Keep your principal staked while using rewards for purchases

## 📌 Devnet Deployment

Program ID: `2pkRfHqNKo5f5rnJaUTAKLkZwXTjVJgsak59Keq1k4MF`

[View on Solana Explorer](https://explorer.solana.com/address/2pkRfHqNKo5f5rnJaUTAKLkZwXTjVJgsak59Keq1k4MF?cluster=devnet)

## ✨ Features

### Core Functionality

* 💰 **Token Staking** - Stake whitelisted tokens to earn yield
* 📈 **Yield Generation** - Automatic APY-based yield calculation
* 🏪 **Merchant Payments** - Pay merchants using accumulated yield
* 🔄 **Flexible Staking** - Add or remove stake at any time
* 🎁 **Yield Claiming** - Claim accumulated yield periodically
* 🔐 **Secure Vaults** - Token vaults with PDA-based security
* ✅ **Token Whitelisting** - Admin-controlled token support

### Future Features (Coming Soon)

* 🤖 **Auto-Pay with Clockwork** - Automated recurring payments to merchants
* 🎯 **Auto-Yield Minting** - Scheduled yield generation using Clockwork
* 📅 **Merchant Subscriptions** - Users can subscribe to merchant services with auto-payments
* 🔔 **Payment Notifications** - Real-time payment confirmation system
* 📊 **Analytics Dashboard** - Track staking performance and payment history

### Technical Highlights

* Built with Anchor framework for type safety
* Optimized for Solana's high throughput
* Comprehensive error handling and validation
* Time-based yield calculation
* Multiple token support with isolated vaults
* Rent-optimized account closures

## 🏗️ Architecture

The protocol consists of several key components:

### State Accounts

* **Config**: Global configuration storing admin, yield parameters, APY, and limits
* **WhitelistToken**: List of supported tokens that can be staked
* **Vault**: Token vaults for each whitelisted token with isolated accounting
* **UserAccount**: Individual user data tracking total stake and yield
* **MerchantAccount**: Merchant profile with business information and payment tracking
* **StakeAccount**: Individual stake positions per user per token

### Core Instructions

1. **Initialize** - Set up global configuration with yield parameters
2. **Whitelist Token** - Admin adds new supported tokens
3. **Onboard User** - Register new users to the platform
4. **Onboard Merchant** - Register merchants with business details
5. **Initialize Stake** - Create first stake position for a token
6. **Add Stake** - Increase existing stake amount
7. **Claim Yield** - Collect accumulated yield rewards
8. **Pay Merchant** - Transfer yield tokens to merchants
9. **Unstake** - Withdraw staked tokens
10. **Close Stake Account** - Reclaim rent after full unstake

## 🚀 Quick Start

### Prerequisites

Ensure you have the following installed:

* [Rust](https://rustup.rs/) (stable toolchain)
* [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.17+)
* [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.29+)
* [Node.js](https://nodejs.org/) (v18+) & npm/yarn

### Installation

1. Clone the repository

```bash
git clone https://github.com/solana-turbin3/Q4_25_Builder_Lovegupta112/tree/main/capstone
cd yieldpay
```

2. Install dependencies

```bash
yarn install
```

3. Build the program

```bash
anchor build
```

4. Run tests

```bash
anchor test
```

### Deployment

To deploy to devnet:

```bash
anchor deploy --provider.cluster devnet
```

To deploy to mainnet-beta:

```bash
anchor deploy --provider.cluster mainnet-beta
```

## 📁 Project Structure

```
yieldpay/
├── programs/
│   └── yieldpay/
│       ├── src/
│       │   ├── instructions/          # Instruction handlers
│       │   │   ├── initialize_config.rs
│       │   │   ├── whitelist_token.rs
│       │   │   ├── onboard_user.rs
│       │   │   ├── onboard_merchant.rs
│       │   │   ├── stake.rs
│       │   │   ├── claim_yield.rs
│       │   │   ├── pay_merchant.rs
│       │   │   └── unstake.rs
│       │   │   └── close_stake_account.rs
│       │   ├── state/                 # State structs
│       │   │   ├── config.rs
│       │   │   ├── vault.rs
│       │   │   ├── user_account.rs
│       │   │   ├── merchant_account.rs
│       │   │   └── whitelist_token.rs
│       │   │   └── stake_account.rs
│       │   ├── errors.rs              # Custom errors
│       │   └── lib.rs                 # Program entrypoint
│       └── Cargo.toml
├── tests/
│   └── yieldpay.ts                    # Integration tests
├── Anchor.toml                        # Anchor configuration
└── package.json
```

## 🧪 Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/yieldpay.ts

```

### Test Coverage

* ✅ Config initialization
* ✅ Token whitelisting (admin only)
* ✅ User onboarding
* ✅ Merchant onboarding
* ✅ Initial stake creation
* ✅ Additional stake deposits
* ✅ Yield calculation and claiming
* ✅ Merchant payments with yield
* ✅ Unstaking tokens
* ✅ Stake account closure
* ✅ Minimum deposit validation
* ✅ Authorization checks
* ✅ Insufficient funds handling
* ✅ Edge cases and error conditions

## 📖 Usage Examples

### For Users

#### 1. Stake Tokens

```typescript
await program.methods
  .initializeStake(new anchor.BN(10_000_000)) // 10 tokens with 6 decimals
  .accounts({
    user: userPublicKey,
    mintX: tokenMint,
    userAccount: userAccountPda,
    userXAta: userTokenAccount,
    config: configPda,
    stakeAccount: stakeAccountPda,
    // ... other accounts
  })
  .rpc();
```

#### 2. Claim Yield

```typescript
await program.methods
  .claimYield()
  .accounts({
    user: userPublicKey,
    mintX: tokenMint,
    userAccount: userAccountPda,
    stakeAccount: stakeAccountPda,
    yieldMint: yieldMintPda,
    yieldMintUserAta: userYieldAta,
    // ... other accounts
  })
  .rpc();
```

#### 3. Pay Merchant

```typescript
await program.methods
  .payMerchant(new anchor.BN(100_000)) // 0.1 yield tokens
  .accounts({
    user: userPublicKey,
    merchant: merchantPublicKey,
    userAccount: userAccountPda,
    merchantAccount: merchantAccountPda,
    yieldMintUserAta: userYieldAta,
    yieldMintMerchantAta: merchantYieldAta,
    // ... other accounts
  })
  .rpc();
```

### For Merchants

#### 1. Onboard as Merchant

```typescript
await program.methods
  .onboardMerchant("My Coffee Shop")
  .accounts({
    merchant: merchantPublicKey,
    merchantAccount: merchantAccountPda,
    config: configPda,
    yieldMint: yieldMintPda,
    yieldMintMerchantAta: merchantYieldAta,
    // ... other accounts
  })
  .rpc();
```

### For Admins

#### 1. Whitelist New Token

```typescript
await program.methods
  .whitelistToken()
  .accounts({
    admin: adminPublicKey,
    mintX: newTokenMint,
    whitelistedTokens: whitelistPda,
    vaultX: vaultPda,
    vaultXAta: vaultAta,
    config: configPda,
    // ... other accounts
  })
  .rpc();
```

## 📊 Yield Calculation

Yield is calculated using the formula:

```
yield = (staked_amount × apy_bps × time_elapsed) / (10000 × yield_period_base)
```

Where:
- `apy_bps`: Annual Percentage Yield in basis points (e.g., 700 = 7%)
- `time_elapsed`: Seconds since last yield mint
- `yield_period_base`: Base time period for yield calculation (e.g., 31,536,000 for yearly)

## 🔐 Security Considerations

* Admin-only functions protected with authority checks
* PDA-based account derivation for security
* Minimum deposit requirements to prevent spam
* Balance validation before unstaking
* Token account ownership verification
* Comprehensive input validation

## 🛣️ Roadmap

### Phase 1: Core Protocol ✅
- [x] Basic staking mechanism
- [x] Yield generation
- [x] Merchant payments
- [x] Multi-token support

### Phase 2: Automation (Coming Soon)
- [ ] Clockwork integration for auto-payments
- [ ] Scheduled yield minting
- [ ] Subscription management system
- [ ] Payment scheduling

### Phase 3: Enhanced Features
- [ ] Web3 frontend interface
- [ ] Mobile app integration
- [ ] Advanced analytics dashboard
- [ ] Loyalty rewards program
- [ ] Cross-chain bridge support

### Phase 4: Ecosystem Growth
- [ ] Merchant SDK
- [ ] API for third-party integrations
- [ ] Governance token launch
- [ ] DAO formation


## 👥 Team

* **Lovegupta** - Core Developer - [@lovegupta](https://github.com/lovegupta112)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with ❤️ on Solana
