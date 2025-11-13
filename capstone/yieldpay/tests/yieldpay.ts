import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Yieldpay } from "../target/types/yieldpay";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";

// Terminal colors helper
const colors = {
  reset: "\x1b[0m",
  bright: "\x1b[1m",
  dim: "\x1b[2m",
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  magenta: "\x1b[35m",
  cyan: "\x1b[36m",
  white: "\x1b[37m",
  bgGreen: "\x1b[42m",
  bgRed: "\x1b[41m",
  bgBlue: "\x1b[44m",
};

// Console logging helpers
const log = {
  header: (text: string) => {
    console.log(`\n${colors.bright}${colors.cyan}${"═".repeat(80)}${colors.reset}`);
    console.log(`${colors.bright}${colors.cyan}  ${text}${colors.reset}`);
    console.log(`${colors.bright}${colors.cyan}${"═".repeat(80)}${colors.reset}\n`);
  },
  
  section: (text: string) => {
    console.log(`\n${colors.bright}${colors.blue}▶ ${text}${colors.reset}`);
    console.log(`${colors.dim}${"─".repeat(60)}${colors.reset}`);
  },
  
  success: (text: string) => {
    console.log(`${colors.green}✓${colors.reset} ${text}`);
  },
  
  error: (text: string) => {
    console.log(`${colors.red}✗${colors.reset} ${text}`);
  },
  
  info: (label: string, value: any) => {
    console.log(`  ${colors.dim}${label}:${colors.reset} ${colors.white}${value}${colors.reset}`);
  },
  
  amount: (label: string, value: number, decimals: number = 6) => {
    const formatted = (value / Math.pow(10, decimals)).toFixed(decimals);
    console.log(`  ${colors.dim}${label}:${colors.reset} ${colors.yellow}${formatted}${colors.reset}`);
  },
  
  tx: (signature: string) => {
    console.log(`  ${colors.dim}Tx Signature:${colors.reset} ${colors.magenta}${signature.slice(0, 8)}...${signature.slice(-8)}${colors.reset}`);
  },
  
  address: (label: string, address: string) => {
    console.log(`  ${colors.dim}${label}:${colors.reset} ${colors.cyan}${address.slice(0, 8)}...${address.slice(-8)}${colors.reset}`);
  },
  
  divider: () => {
    console.log(`${colors.dim}${"─".repeat(80)}${colors.reset}`);
  },
  
  space: () => {
    console.log("");
  }
};

describe("yieldpay", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.yieldpay as Program<Yieldpay>;

  const admin = provider.wallet;
  const user = anchor.web3.Keypair.generate();
  const merchant = anchor.web3.Keypair.generate();

  let userAccountPda: anchor.web3.PublicKey;
  let merchantAccountPda: anchor.web3.PublicKey;
  let configPda: anchor.web3.PublicKey;
  let configBump: number;
  let yieldMint: anchor.web3.PublicKey;
  let yieldMintBump: number;
  let yieldMintUserAta: anchor.web3.PublicKey;
  let yieldMintMerchantAta: anchor.web3.PublicKey;
  let whitelistedTokenPda: anchor.web3.PublicKey;
  let whitelistedTokenBump: number;

  let vaultXPda: anchor.web3.PublicKey;
  let vaultXBump: number;
  let vaultXAta: anchor.web3.PublicKey;
  let mintX: anchor.web3.PublicKey;
  let userXAta: anchor.web3.PublicKey;

  let vaultYPda: anchor.web3.PublicKey;
  let vaultYBump: number;
  let vaultYAta: anchor.web3.PublicKey;
  let mintY: anchor.web3.PublicKey;
  let userYAta: anchor.web3.PublicKey;

  let stakeXAccount: anchor.web3.PublicKey;
  let stakeXBump: number;
  let stakeYAccount: anchor.web3.PublicKey;
  let stakeYBump: number;

  let max_stake = new anchor.BN(50_000_000);
  let min_deposit = new anchor.BN(5_000_000);
  let total_users = new anchor.BN(0);
  let total_merchants = new anchor.BN(0);
  let yield_min_period = new anchor.BN(1);
  let apy_bps = new anchor.BN(700);
  const busineesNameA = "Merchant A private ltd";
  const initalAmount = new anchor.BN(20_000_000);
  const yield_period_base = new anchor.BN(60);

  const base = 1_000_000;

  const CONFIG_SEED = "CONFIG";
  const YIELD_MINT_SEED = "YIELD";
  const WHITELIST_SEED = "WHITELIST";
  const VAULT_SEED = "VAULT";
  const USER_SEED = "USER";
  const MERCHANT_SEED = "MERCHANT";
  const STAKE_SEED = "STAKE";

  log.header("YIELDPAY TEST SUITE INITIALIZATION");
  
  log.section("Account Addresses");
  log.address("Admin", admin.publicKey.toString());
  log.address("User", user.publicKey.toString());
  log.address("Merchant", merchant.publicKey.toString());
  
  log.section("Initial Configuration");
  log.amount("Max Stake", max_stake.toNumber());
  log.amount("Min Deposit", min_deposit.toNumber());
  log.info("Total Users", total_users.toString());
  log.info("Total Merchants", total_merchants.toString());
  log.info("Yield Min Period (seconds)", yield_min_period.toString());
  log.info("APY (basis points)", apy_bps.toString());
  log.info("Yield Period Base (seconds)", yield_period_base.toString());

  before(async () => {
    log.header("🔧 SETUP: Airdropping SOL & Deriving PDAs");
    
    await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      user.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      merchant.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    log.success("Airdropped 10 SOL to each account");

    [configPda, configBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(CONFIG_SEED)],
      program.programId
    );
    log.section("Config PDA");
    log.address("Config PDA", configPda.toString());
    log.info("Config Bump", configBump);

    [userAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_SEED), user.publicKey.toBuffer()],
      program.programId
    );
    [merchantAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MERCHANT_SEED), merchant.publicKey.toBuffer()],
      program.programId
    );
    log.section("User & Merchant PDAs");
    log.address("User Account PDA", userAccountPda.toString());
    log.address("Merchant Account PDA", merchantAccountPda.toString());

    [yieldMint, yieldMintBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(YIELD_MINT_SEED), configPda.toBuffer()],
      program.programId
    );
    log.section("Yield Mint");
    log.address("Yield Mint", yieldMint.toString());
    log.info("Yield Mint Bump", yieldMintBump);

    yieldMintUserAta = getAssociatedTokenAddressSync(
      yieldMint,
      userAccountPda,
      true
    );
    yieldMintMerchantAta = getAssociatedTokenAddressSync(
      yieldMint,
      merchantAccountPda,
      true
    );
    log.address("User Yield ATA", yieldMintUserAta.toString());
    log.address("Merchant Yield ATA", yieldMintMerchantAta.toString());

    [whitelistedTokenPda, whitelistedTokenBump] =
      PublicKey.findProgramAddressSync(
        [Buffer.from(WHITELIST_SEED), configPda.toBuffer()],
        program.programId
      );
    log.section("Whitelist");
    log.address("Whitelist PDA", whitelistedTokenPda.toString());
    log.info("Whitelist Bump", whitelistedTokenBump);

    mintX = await createMint(connection, admin.payer, admin.publicKey, null, 6);
    log.section("Token X (Mint & Vault)");
    log.address("Mint X", mintX.toString());

    [vaultXPda, vaultXBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED), mintX.toBuffer(), configPda.toBuffer()],
      program.programId
    );
    vaultXAta = getAssociatedTokenAddressSync(mintX, vaultXPda, true);
    log.address("Vault X PDA", vaultXPda.toString());
    log.info("Vault X Bump", vaultXBump);
    log.address("Vault X ATA", vaultXAta.toString());

    mintY = await createMint(connection, admin.payer, admin.publicKey, null, 0);
    log.section("Token Y (Mint & Vault)");
    log.address("Mint Y", mintY.toString());

    [vaultYPda, vaultYBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED), mintY.toBuffer(), configPda.toBuffer()],
      program.programId
    );
    vaultYAta = getAssociatedTokenAddressSync(mintY, vaultYPda, true);
    log.address("Vault Y PDA", vaultYPda.toString());
    log.info("Vault Y Bump", vaultYBump);
    log.address("Vault Y ATA", vaultYAta.toString());

    const userXAtaInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mintX,
      user.publicKey
    );
    userXAta = userXAtaInfo.address;
    await mintTo(
      provider.connection,
      user,
      mintX,
      userXAta,
      provider.wallet.payer,
      initalAmount.toNumber()
    );
    log.section("User Token X Setup");
    log.address("User X ATA", userXAta.toString());
    log.amount("Minted to User", initalAmount.toNumber());

    [stakeXAccount, stakeXBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(STAKE_SEED),
        configPda.toBuffer(),
        mintX.toBuffer(),
        userAccountPda.toBuffer(),
      ],
      program.programId
    );
    log.address("Stake X Account", stakeXAccount.toString());
    log.info("Stake X Bump", stakeXBump);

    const userYAtaInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mintY,
      user.publicKey
    );
    userYAta = userYAtaInfo.address;
    await mintTo(
      provider.connection,
      user,
      mintY,
      userYAta,
      provider.wallet.payer,
      initalAmount.toNumber()
    );
    log.section("User Token Y Setup");
    log.address("User Y ATA", userYAta.toString());
    log.amount("Minted to User", initalAmount.toNumber(), 0);

    [stakeYAccount, stakeYBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(STAKE_SEED),
        configPda.toBuffer(),
        mintY.toBuffer(),
        userAccountPda.toBuffer(),
      ],
      program.programId
    );
    log.address("Stake Y Account", stakeYAccount.toString());
    log.info("Stake Y Bump", stakeYBump);
  });

  it("config is initialized!", async () => {
    log.header(" TEST 1: Initialize Config");
    
    const tx = await program.methods
      .initialize({
        maxStake: max_stake,
        minDeposit: min_deposit,
        totalUsers: total_users,
        totalMerchants: total_merchants,
        yieldMinPeriod: yield_min_period,
        apyBps: apy_bps,
        yieldPeriodBase: yield_period_base,
      })
      .accountsStrict({
        admin: admin.publicKey,
        config: configPda,
        yieldMint: yieldMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    log.tx(tx);
    const configAccount = await program.account.config.fetch(configPda);

    log.section("Verification");
    expect(configAccount.admin.toBase58()).to.equal(admin.publicKey.toBase58());
    log.success("Admin matches");
    expect(configAccount.yieldMint.toBase58()).to.equal(yieldMint.toBase58());
    log.success("Yield mint matches");
    expect(configAccount.configBump).to.equal(configBump);
    log.success("Config bump matches");
    expect(configAccount.yieldBump).to.equal(yieldMintBump);
    log.success("Yield bump matches");
    expect(configAccount.apyBps.toNumber()).to.equal(apy_bps.toNumber());
    log.success("APY BPS matches");
    expect(configAccount.totalUsers.toNumber()).to.equal(total_users.toNumber());
    log.success("Total users initialized");
    expect(configAccount.totalMerchants.toNumber()).to.equal(total_merchants.toNumber());
    log.success("Total merchants initialized");
    expect(configAccount.maxStake.toNumber()).to.equal(max_stake.toNumber());
    log.success("Max stake matches");
    expect(configAccount.minDeposit.toNumber()).to.equal(min_deposit.toNumber());
    log.success("Min deposit matches");
    expect(configAccount.yieldMinPeriod.toString()).to.equal(yield_min_period.toString());
    log.success("Yield min period matches");
  });

  it("Admin whitelists the token X ", async () => {
    log.header("TEST 2: Whitelist Token X");
    
    const tx1 = await program.methods
      .whitelistToken()
      .accountsStrict({
        admin: admin.publicKey,
        mintX: mintX,
        whitelistedTokens: whitelistedTokenPda,
        vaultX: vaultXPda,
        vaultXAta: vaultXAta,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    log.tx(tx1);

    const whitelistTokenAccount = await program.account.whitelistToken.fetch(
      whitelistedTokenPda
    );
    const vaultXPdaAccount = await program.account.vault.fetch(vaultXPda);

    log.section("Verification");
    expect(whitelistTokenAccount.tokens.find((key) => key.equals(mintX))).to.not.be.undefined;
    log.success("Token X added to whitelist");
    expect(whitelistTokenAccount.supportedTokenNum).to.equal(1);
    log.info("Supported tokens count", 1);
    expect(whitelistTokenAccount.bump).to.equal(whitelistedTokenBump);
    log.success("Whitelist bump matches");
    expect(vaultXPdaAccount.mint.toBase58()).to.equal(mintX.toBase58());
    log.success("Vault X mint matches");
    expect(vaultXPdaAccount.tokenAccount.toBase58()).to.equal(vaultXAta.toBase58());
    log.success("Vault X token account matches");
  });

  it("User can't whitelist the token X", async () => {
    log.header("🚫 TEST 3: User Cannot Whitelist (Negative Test)");
    
    try {
      const tx = await program.methods
        .whitelistToken()
        .accountsStrict({
          admin: user.publicKey,
          mintX: mintY,
          whitelistedTokens: whitelistedTokenPda,
          vaultX: vaultYPda,
          vaultXAta: vaultYAta,
          config: configPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();

      log.error("Test failed - User was able to whitelist!");
      expect.fail("User was trying to whitelist the token Y");
    } catch (err) {
      log.success("Correctly blocked unauthorized whitelist attempt");
      expect(err.error.errorMessage).to.equal(
        "Unauthorized access: this account is not the owner or authorized authority."
      );
      log.info("Error message", err.error.errorMessage);
    }
  });

  it("Onboards the User", async () => {
    log.header("TEST 4: Onboard User");
    
    const tx = await program.methods
      .onboardUser()
      .accountsStrict({
        user: user.publicKey,
        userAccount: userAccountPda,
        config: configPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    log.tx(tx);

    const userAccountInfo = await program.account.userAccount.fetch(userAccountPda);
    const configAccount = await program.account.config.fetch(configPda);

    log.section("Verification");
    expect(userAccountInfo.totalAmountStaked.toNumber()).to.equal(0);
    log.success("Total staked initialized to 0");
    expect(userAccountInfo.totalYield.toNumber()).to.equal(0);
    log.success("Total yield initialized to 0");
    expect(userAccountInfo.owner.toBase58()).to.equal(user.publicKey.toBase58());
    log.success("User owner matches");
    expect(configAccount.totalUsers.toNumber()).to.equal(1);
    log.info("Total users", 1);
  });

  it("Onboards the Merchant", async () => {
    log.header(" TEST 5: Onboard Merchant");
    
    const tx = await program.methods
      .onboardMerchant(busineesNameA)
      .accountsStrict({
        merchant: merchant.publicKey,
        merchantAccount: merchantAccountPda,
        config: configPda,
        yieldMint: yieldMint,
        yieldMintMerchantAta: yieldMintMerchantAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([merchant])
      .rpc();

    log.tx(tx);

    const merchantAccountInfo = await program.account.merchantAccount.fetch(merchantAccountPda);
    const configAccount = await program.account.config.fetch(configPda);

    log.section("Verification");
    expect(merchantAccountInfo.businessName).to.equal(busineesNameA);
    log.info("Business name", busineesNameA);
    expect(merchantAccountInfo.totalReceived.toNumber()).to.equal(0);
    log.success("Total received initialized to 0");
    expect(merchantAccountInfo.owner.toBase58()).to.equal(merchant.publicKey.toBase58());
    log.success("Merchant owner matches");
    expect(configAccount.totalMerchants.toNumber()).to.equal(1);
    log.info("Total merchants", 1);
  });

  it("User stakes 10 tokens first time", async () => {
    log.header("TEST 6: Initial Stake (10 Tokens)");
    
    const userXAtaInfoBefore = await getAccount(connection, userXAta);
    const vaultXAtaInfoBefore = await getAccount(connection, vaultXAta);

    log.section("Balances Before Staking");
    log.amount("User X Token", Number(userXAtaInfoBefore.amount));
    log.amount("Vault X", Number(vaultXAtaInfoBefore.amount));

    const tx = await program.methods
      .initializeStake(new anchor.BN(10_000_000))
      .accountsStrict({
        user: user.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        userXAta: userXAta,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        vaultX: vaultXPda,
        vaultXAta: vaultXAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    
    log.tx(tx);

    const userStakeXaccount = await program.account.stakeAccount.fetch(stakeXAccount);
    const userXAtaInfoAfter = await getAccount(connection, userXAta);
    const vaultXAtaInfoAfter = await getAccount(connection, vaultXAta);

    log.section("Balances After Staking");
    log.amount("User X Token", Number(userXAtaInfoAfter.amount));
    log.amount("Vault X", Number(vaultXAtaInfoAfter.amount));

    log.section("Verification");
    assert.ok(vaultXAtaInfoBefore.amount < vaultXAtaInfoAfter.amount, "Vault X balance should increase");
    log.success("Vault balance increased");
    expect(userStakeXaccount.amountStaked.toNumber()).to.equals(10_000_000);
    log.amount("Amount staked", 10_000_000);
    expect(userStakeXaccount.isActive).to.equals(true);
    log.success("Stake account is active");
    expect(userStakeXaccount.owner.toBase58()).to.equals(userAccountPda.toBase58());
    log.success("Stake owner matches");
    expect(userStakeXaccount.mint.toBase58()).to.equals(mintX.toBase58());
    log.success("Stake mint matches");
  });

  it("Should fail if user stakes 3 tokens less than min deposit (5) ", async () => {
    log.header("🚫 TEST 7: Stake Below Minimum (Negative Test)");
    
    try {
      const tx = await program.methods
        .addStake(new anchor.BN(3_000_000))
        .accountsStrict({
          user: user.publicKey,
          mintX: mintX,
          userAccount: userAccountPda,
          userXAta: userXAta,
          config: configPda,
          stakeAccount: stakeXAccount,
          whitelistedTokens: whitelistedTokenPda,
          vaultX: vaultXPda,
          vaultXAta: vaultXAta,
          yieldMint: yieldMint,
          yieldMintUserAta: yieldMintUserAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();
      log.error("Test failed - User staked below minimum!");
      expect.fail(`User was staking 3 token less than min deposit ${min_deposit}`);
    } catch (err) {
      log.success("Correctly rejected stake below minimum");
      expect(err.error.errorCode.code).to.equals("DepositTooSmall");
      log.info("Error code", "DepositTooSmall");
    }
  });

  it("user stakes 5 tokens again", async () => {
    log.header("TEST 8: Additional Stake (5 Tokens)");
    
    const userXAtaInfoBefore = await getAccount(connection, userXAta);
    const vaultXAtaInfoBefore = await getAccount(connection, vaultXAta);

    log.section("Balances Before Staking");
    log.amount("User X Token", Number(userXAtaInfoBefore.amount));
    log.amount("Vault X", Number(vaultXAtaInfoBefore.amount));
    
    log.info("Waiting", "3 seconds for yield to accrue...");
    await new Promise((resolve) => setTimeout(resolve, 3000));

    const tx = await program.methods
      .addStake(new anchor.BN(5_000_000))
      .accountsStrict({
        user: user.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        userXAta: userXAta,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        vaultX: vaultXPda,
        vaultXAta: vaultXAta,
        yieldMint: yieldMint,
        yieldMintUserAta: yieldMintUserAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const userStakeXaccount = await program.account.stakeAccount.fetch(stakeXAccount);
    log.tx(tx);
    
    log.section("Stake Account Details");
    log.info("Staked at", new Date(userStakeXaccount.stakedAt.toNumber() * 1000).toLocaleString());
    log.amount("Total yield accumulated", userStakeXaccount.totalYield?.toNumber() || 0);
    log.info("Last yield mint", userStakeXaccount.lastYieldMint?.toNumber() || 0);

    const userXAtaInfoAfter = await getAccount(connection, userXAta);
    const vaultXAtaInfoAfter = await getAccount(connection, vaultXAta);
    const yieldMintUserAtaAfter = await getAccount(connection, yieldMintUserAta);

    log.section("Balances After Staking");
    log.amount("User X Token", Number(userXAtaInfoAfter.amount));
    log.amount("Vault X", Number(vaultXAtaInfoAfter.amount));
    log.amount("User Yield", Number(yieldMintUserAtaAfter.amount));

    log.section("Verification");
    expect(userStakeXaccount.amountStaked.toNumber()).to.equals(15_000_000);
    log.amount("Total staked", 15_000_000);
    expect(userStakeXaccount.isActive).to.equals(true);
    log.success("Stake account is active");
    expect(userStakeXaccount.owner.toBase58()).to.equals(userAccountPda.toBase58());
    log.success("Stake owner matches");
    expect(userStakeXaccount.mint.toBase58()).to.equals(mintX.toBase58());
    log.success("Stake mint matches");
    expect(userStakeXaccount.totalYield.toNumber()).to.not.equals(0);
    log.success("Yield has accrued");
  });

  it("user claims yield after min period", async () => {
    log.header("TEST 9: Claim Yield");
    
    const yieldMintUserAtaBefore = await getAccount(connection, yieldMintUserAta);

    log.section("Balance Before Claiming");
    log.amount("User Yield", Number(yieldMintUserAtaBefore.amount));
    
    log.info("Waiting", "3 seconds for additional yield...");
    await new Promise((resolve) => setTimeout(resolve, 3000));

    const tx = await program.methods
      .claimYield()
      .accountsStrict({
        user: user.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        yieldMint: yieldMint,
        yieldMintUserAta: yieldMintUserAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    log.tx(tx);

    const yieldMintUserAtaAfter = await getAccount(connection, yieldMintUserAta);

    log.section("Balance After Claiming");
    log.amount("User Yield", Number(yieldMintUserAtaAfter.amount));

    log.section("Verification");
    assert.ok(
      Number(yieldMintUserAtaAfter.amount) / 6 > Number(yieldMintUserAtaBefore.amount) / 6,
      "User's yield balance should increase after claiming"
    );
    log.success("Yield balance increased");
    expect(Number(yieldMintUserAtaAfter.amount) / base).to.not.equals(0);
    log.success("User has claimable yield");
  });

  it("user can't claim yield if there is no stake account exist", async () => {
    log.header("🚫 TEST 10: Cannot Claim Without Stake (Negative Test)");

    log.section("Whitelisting Token Y");
    const tx2 = await program.methods
      .whitelistToken()
      .accountsStrict({
        admin: admin.publicKey,
        mintX: mintY,
        whitelistedTokens: whitelistedTokenPda,
        vaultX: vaultYPda,
        vaultXAta: vaultYAta,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();
    log.tx(tx2);

    const whitelistTokenAccount = await program.account.whitelistToken.fetch(
      whitelistedTokenPda
    );
    const vaultYPdaAccount = await program.account.vault.fetch(vaultYPda);
    
    expect(vaultYPdaAccount.mint.toBase58()).to.equal(mintY.toBase58());
    log.success("Vault Y mint matches");
    expect(vaultYPdaAccount.tokenAccount.toBase58()).to.equal(vaultYAta.toBase58());
    log.success("Vault Y token account matches");
    expect(whitelistTokenAccount.tokens.find((key) => key.equals(mintY))).to.not.be.undefined;
    log.success("Token Y added to whitelist");
    expect(whitelistTokenAccount.supportedTokenNum).to.equal(2);
    log.info("Total whitelisted tokens", 2);

    log.section("Attempting to Claim Without Stake");
    try {
      const tx = await program.methods
        .claimYield()
        .accountsStrict({
          user: user.publicKey,
          mintX: mintY,
          userAccount: userAccountPda,
          config: configPda,
          stakeAccount: stakeYAccount,
          whitelistedTokens: whitelistedTokenPda,
          yieldMint: yieldMint,
          yieldMintUserAta: yieldMintUserAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();

      log.error("Test failed - User claimed without stake!");
      expect.fail(
        `user tried to claim yield if there is no stake amount exist for that mintY ${mintY.toString()}`
      );
    } catch (err) {
      log.success("Correctly blocked claim without stake");
      expect(err.error.errorCode.code).to.equals("AccountNotInitialized");
      log.info("Error code", "AccountNotInitialized");
    }
  });

  it("user can pay yield to merchant", async () => {
    log.header("TEST 11: Pay Merchant with Yield");

    const yieldMintUserAtaBefore = await getAccount(connection, yieldMintUserAta);
    const yieldMintMerchantAtaBefore = await getAccount(connection, yieldMintMerchantAta);

    log.section("Balances Before Payment");
    log.amount("User Yield", Number(yieldMintUserAtaBefore.amount));
    log.amount("Merchant Yield", Number(yieldMintMerchantAtaBefore.amount));

    const paymentAmount = new anchor.BN(100);
    const tx = await program.methods
      .payMerchant(paymentAmount)
      .accountsStrict({
        user: user.publicKey,
        merchant: merchant.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        merchantAccount: merchantAccountPda,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        yieldMint: yieldMint,
        yieldMintUserAta: yieldMintUserAta,
        yieldMintMerchantAta: yieldMintMerchantAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    log.tx(tx);

    const yieldMintUserAtaAfter = await getAccount(connection, yieldMintUserAta);
    const yieldMintMerchantAtaAfter = await getAccount(connection, yieldMintMerchantAta);

    log.section("Balances After Payment");
    log.amount("User Yield", Number(yieldMintUserAtaAfter.amount));
    log.amount("Merchant Yield", Number(yieldMintMerchantAtaAfter.amount));
    log.amount("Payment Amount", paymentAmount.toNumber());

    log.section("Verification");
    assert.ok(
      Number(yieldMintUserAtaAfter.amount) < Number(yieldMintUserAtaBefore.amount),
      "User's yield balance should decrease after paying"
    );
    log.success("User balance decreased");
    assert.ok(
      Number(yieldMintMerchantAtaAfter.amount) > Number(yieldMintMerchantAtaBefore.amount),
      "Merchant's yield balance should increase after getting payment"
    );
    log.success("Merchant balance increased");
  });

  it("user can't pay more than existing yield to merchant", async () => {
    log.header("🚫 TEST 12: Cannot Overpay (Negative Test)");

    const yieldMintUserAtaBefore = await getAccount(connection, yieldMintUserAta);

    log.section("Current Balance");
    log.amount("User Yield", Number(yieldMintUserAtaBefore.amount));

    log.section("Attempting to Pay 2 Tokens (More Than Available)");
    try {
      await program.methods
        .payMerchant(new anchor.BN(2_000_000))
        .accountsStrict({
          user: user.publicKey,
          merchant: merchant.publicKey,
          mintX: mintX,
          userAccount: userAccountPda,
          merchantAccount: merchantAccountPda,
          config: configPda,
          stakeAccount: stakeXAccount,
          whitelistedTokens: whitelistedTokenPda,
          yieldMint: yieldMint,
          yieldMintUserAta: yieldMintUserAta,
          yieldMintMerchantAta: yieldMintMerchantAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();

      log.error("Test failed - User overpaid!");
      expect.fail(
        `user tried to paid yield more than existing yield: ${yieldMintUserAta.toString()}`
      );
    } catch (err) {
      log.success("Correctly rejected overpayment");
      expect(err.error.errorCode.code).to.equals("InsufficientFunds");
      log.info("Error code", "InsufficientFunds");
    }
  });

  it("user can unstake existing 10 stake X tokens", async () => {
    log.header("TEST 13: Unstake Tokens");

    const userXAtaInfoBefore = await getAccount(connection, userXAta);
    const userStakeXAtaInfoBefore = await program.account.stakeAccount.fetch(stakeXAccount);
    const vaultXInfoBefore = await program.account.vault.fetch(vaultXPda);

    log.section("Balances Before Unstaking");
    log.amount("User X Token", Number(userXAtaInfoBefore.amount));
    log.amount("Vault X Total Staked", vaultXInfoBefore.totalAmountStaked.toNumber());
    log.amount("User Stake Account", userStakeXAtaInfoBefore.amountStaked.toNumber());

    const unstakeAmount = new anchor.BN(10_000_000);
    const tx = await program.methods
      .unstake(unstakeAmount)
      .accountsStrict({
        user: user.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        userXAta: userXAta,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        vaultX: vaultXPda,
        vaultXAta: vaultXAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    log.tx(tx);

    const userXAtaInfoAfter = await getAccount(connection, userXAta);
    const vaultXInfoAfter = await program.account.vault.fetch(vaultXPda);
    const userStakeXAtaInfoAfter = await program.account.stakeAccount.fetch(stakeXAccount);

    log.section("Balances After Unstaking");
    log.amount("User X Token", Number(userXAtaInfoAfter.amount));
    log.amount("Vault X Total Staked", vaultXInfoAfter.totalAmountStaked.toNumber());
    log.amount("User Stake Account", userStakeXAtaInfoAfter.amountStaked.toNumber());

    log.section("Verification");
    assert.ok(
      Number(userXAtaInfoAfter.amount) > Number(userXAtaInfoBefore.amount),
      "User's x ata token balance should increase after unstake"
    );
    log.success("User balance increased");
    assert.ok(
      vaultXInfoAfter.totalAmountStaked.toNumber() < vaultXInfoBefore.totalAmountStaked.toNumber(),
      "vault X balance should decrease after unstake"
    );
    log.success("Vault balance decreased");
  });

  it("user can't unstake more than existing stake amount", async () => {
    log.header("🚫 TEST 14: Cannot Unstake More Than Staked (Negative Test)");

    const userXAtaInfoBefore = await getAccount(connection, userXAta);
    const userStakeXAtaInfoBefore = await program.account.stakeAccount.fetch(stakeXAccount);
    const vaultXInfoBefore = await program.account.vault.fetch(vaultXPda);

    log.section("Current State");
    log.amount("User X Token", Number(userXAtaInfoBefore.amount));
    log.amount("Vault X Total Staked", vaultXInfoBefore.totalAmountStaked.toNumber());
    log.amount("User Stake Account", userStakeXAtaInfoBefore.amountStaked.toNumber());

    log.section("Attempting to Unstake 50 Tokens (More Than Staked)");
    try {
      const tx = await program.methods
        .unstake(new anchor.BN(50_000_000))
        .accountsStrict({
          user: user.publicKey,
          mintX: mintX,
          userAccount: userAccountPda,
          userXAta: userXAta,
          config: configPda,
          stakeAccount: stakeXAccount,
          whitelistedTokens: whitelistedTokenPda,
          vaultX: vaultXPda,
          vaultXAta: vaultXAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();
      log.error("Test failed - User unstaked more than available!");
    } catch (err) {
      log.success("Correctly rejected excessive unstake");
      expect(err.error.errorCode.code).to.equals("InvalidAmount");
      log.info("Error code", "InvalidAmount");
    }
  });

  it("user can't close stake X account before unstaking fully", async () => {
    log.header("🚫 TEST 15: Cannot Close With Active Stake (Negative Test)");

    const userStakeXAtaInfoBefore = await program.account.stakeAccount.fetch(stakeXAccount);
    const userBalanceBefore = await connection.getBalance(user.publicKey);

    log.section("Current Stake Account State");
    log.info("Is Active", userStakeXAtaInfoBefore.isActive);
    log.amount("Amount Staked", userStakeXAtaInfoBefore.amountStaked.toNumber());

    try {
      const tx = await program.methods
        .closeStakeAccount()
        .accountsStrict({
          user: user.publicKey,
          mintX: mintX,
          userAccount: userAccountPda,
          userXAta: userXAta,
          config: configPda,
          stakeAccount: stakeXAccount,
          whitelistedTokens: whitelistedTokenPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();
      log.error("Test failed - Stake account closed with active stake!");
      assert.fail("Stake account should be closed");
    } catch (err) {
      log.success("Correctly blocked closure with active stake");
      expect(err.error.errorCode.code).to.equals("MustUnstakeFirst");
      log.info("Error code", "MustUnstakeFirst");
    }
  });

  it("user can close stake x account after full stake", async () => {
    log.header("TEST 16: Close Stake Account After Full Unstake");

    const userStakeXAtaInfoBefore = await program.account.stakeAccount.fetch(stakeXAccount);
    const left_amount = userStakeXAtaInfoBefore.amountStaked;

    log.section("Unstaking Remaining Amount");
    log.info("Is Active", userStakeXAtaInfoBefore.isActive);
    log.amount("Remaining Staked", left_amount.toNumber());

    const tx1 = await program.methods
      .unstake(left_amount)
      .accountsStrict({
        user: user.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        userXAta: userXAta,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        vaultX: vaultXPda,
        vaultXAta: vaultXAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    log.tx(tx1);

    const userStakeXAtaInfoAfter = await program.account.stakeAccount.fetch(stakeXAccount);

    log.section("Stake Account After Full Unstake");
    log.info("Is Active", userStakeXAtaInfoAfter.isActive);
    log.amount("Amount Staked", userStakeXAtaInfoAfter.amountStaked.toNumber());

    expect(userStakeXAtaInfoAfter.amountStaked.toNumber()).to.equals(0);
    log.success("All tokens unstaked");
    expect(userStakeXAtaInfoAfter.isActive).to.be.false;
    log.success("Stake account deactivated");

    const userBalanceBefore = await connection.getBalance(user.publicKey);
    log.section("Closing Stake Account");
    log.info("User SOL balance before", (userBalanceBefore / anchor.web3.LAMPORTS_PER_SOL).toFixed(4));

    const tx2 = await program.methods
      .closeStakeAccount()
      .accountsStrict({
        user: user.publicKey,
        mintX: mintX,
        userAccount: userAccountPda,
        userXAta: userXAta,
        config: configPda,
        stakeAccount: stakeXAccount,
        whitelistedTokens: whitelistedTokenPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    log.tx(tx2);

    try {
      await program.account.stakeAccount.fetch(stakeXAccount);
      log.error("Stake account still exists!");
      assert.fail("Stake account should have been closed but still exists");
    } catch (err) {
      assert.include(err.message, "Account does not exist");
      log.success("Stake account closed successfully");
    }

    const userBalanceAfter = await connection.getBalance(user.publicKey);
    log.info("User SOL balance after", (userBalanceAfter / anchor.web3.LAMPORTS_PER_SOL).toFixed(4));
    
    assert.ok(
      userBalanceAfter > userBalanceBefore,
      "User should receive rent refund after closing stake account"
    );
    log.success("Rent refund received");
    
    log.header("✅ ALL TESTS COMPLETED SUCCESSFULLY");
  });
});