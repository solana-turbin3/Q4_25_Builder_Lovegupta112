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
  let stakeXAccount: anchor.web3.PublicKey;
  let stakeXBump: number;
  let max_stake = new anchor.BN(50);
  let min_deposit = new anchor.BN(5);
  let total_users = new anchor.BN(0);
  let total_merchants = new anchor.BN(0);
  let yield_min_period = new anchor.BN(1 * 24 * 60 * 60); //1 day
  let apy_bps = new anchor.BN(700);
  const busineesNameA = "Merchant A private ltd";
  const initalAmount = new anchor.BN(20);

  const CONFIG_SEED = "CONFIG";
  const YIELD_MINT_SEED = "YIELD";
  const WHITELIST_SEED = "WHITELIST";
  const VAULT_SEED = "VAULT";
  const USER_SEED = "USER";
  const MERCHANT_SEED = "MERCHANT";
  const STAKE_SEED = "STAKE";

  console.log("-------------------------------\n");
  console.log("Admin pubkey: ", admin.publicKey.toString());
  console.log("User pubkey: ", user.publicKey.toString());
  console.log("Merchant pubkey: ", merchant.publicKey.toString());
  console.log("\n📊 Initial configuration values:");
  console.log("-------------------------------");
  console.log("🪙 max_stake:", max_stake.toString());
  console.log("💰 min_deposit:", min_deposit);
  console.log("👥 total_users:", total_users.toString());
  console.log("🏪 total_merchants:", total_merchants.toString());
  console.log("⏳ yield_min_period (days):", yield_min_period.toString());
  console.log("📈 apy_bps:", apy_bps);
  console.log("-------------------------------\n");

  before(async () => {
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

    [configPda, configBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(CONFIG_SEED)],
      program.programId
    );

    console.log(
      `ConfigPda: ${configPda.toString()} and configBump: ${configBump}`
    );

    [userAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_SEED), user.publicKey.toBuffer()],
      program.programId
    );
    [merchantAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(MERCHANT_SEED), merchant.publicKey.toBuffer()],
      program.programId
    );

    console.log(
      `userAccount: ${userAccountPda.toString()} , merchantAccount: ${merchantAccountPda.toString()}`
    );

    [yieldMint, yieldMintBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(YIELD_MINT_SEED), configPda.toBuffer()],
      program.programId
    );

    console.log(
      `yieldMint: ${yieldMint.toString()} and yieldMintBump: ${yieldMintBump}`
    );

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

    console.log(
      `yieldMintUserAta: ${yieldMintUserAta}, yieldMintMerchantAta: ${yieldMintMerchantAta}`
    );

    [whitelistedTokenPda, whitelistedTokenBump] =
      PublicKey.findProgramAddressSync(
        [Buffer.from(WHITELIST_SEED), configPda.toBuffer()],
        program.programId
      );

    console.log(
      `whitelistedTokenPda: ${yieldMint.toString()} and whitelistedTokenBump: ${yieldMintBump}`
    );

    //creating mint that will be whitelisted by admin ---
    mintX = await createMint(connection, admin.payer, admin.publicKey, null, 0);

    console.log("mintX: ", mintX.toString());

    [vaultXPda, vaultXBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED), mintX.toBuffer(), configPda.toBuffer()],
      program.programId
    );

    vaultXAta = getAssociatedTokenAddressSync(mintX, vaultXPda, true);

    console.log(
      `vaultXPda: ${vaultXPda.toString()} , vaultXBump: ${vaultXBump} and its associated vaultX: ${vaultXAta.toString()}`
    );
    //creating mint that will be whitelisted by user ---
    mintY = await createMint(connection, user, user.publicKey, null, 0);
    console.log("mintY: ", mintY.toString());

    [vaultYPda, vaultYBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED), mintY.toBuffer(), configPda.toBuffer()],
      program.programId
    );

    vaultYAta = getAssociatedTokenAddressSync(mintY, vaultYPda, true);

    console.log(
      `vaultYPda: ${vaultYPda.toString()} , vaultYBump: ${vaultYBump} and its associated vaultY: ${vaultYAta.toString()}`
    );
    //Minting some tokens to user -------
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

    [stakeXAccount, stakeXBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(STAKE_SEED),
        configPda.toBuffer(),
        mintX.toBuffer(),
        userAccountPda.toBuffer(),
      ],
      program.programId
    );

    console.log(
      `userXata: ${userXAta?.toString()}, stakeXAccount: ${stakeXAccount.toString()}, stakeXBump: ${stakeXBump}`
    );

    console.log("-------------------------------\n");
  });

  it("config is initialized!", async () => {
    const tx = await program.methods
      .initialize({
        maxStake: max_stake,
        minDeposit: min_deposit,
        totalUsers: total_users,
        totalMerchants: total_merchants,
        yieldMinPeriod: yield_min_period,
        apyBps: apy_bps,
      })
      .accountsStrict({
        admin: admin.publicKey,
        config: configPda,
        yieldMint: yieldMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Config initialized transaction signature", tx);
    const configAccount = await program.account.config.fetch(configPda);

    expect(configAccount.admin.toBase58()).to.equal(admin.publicKey.toBase58());
    expect(configAccount.yieldMint.toBase58()).to.equal(yieldMint.toBase58());
    expect(configAccount.configBump).to.equal(configBump);
    expect(configAccount.yieldBump).to.equal(yieldMintBump);
    expect(configAccount.apyBps.toNumber()).to.equal(apy_bps.toNumber());
    expect(configAccount.totalUsers.toNumber()).to.equal(
      total_users.toNumber()
    );
    expect(configAccount.totalMerchants.toNumber()).to.equal(
      total_merchants.toNumber()
    );
    expect(configAccount.maxStake.toNumber()).to.equal(max_stake.toNumber());
    expect(configAccount.minDeposit.toNumber()).to.equal(
      min_deposit.toNumber()
    );
    expect(configAccount.yieldMinPeriod.toString()).to.equal(
      yield_min_period.toString()
    );
  });

  it("Admin whitelists the token X", async () => {
    const tx = await program.methods
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
    console.log("Admin whitelisted token transaction signature", tx);

    const whitelistTokenAccount = await program.account.whitelistToken.fetch(
      whitelistedTokenPda
    );
    const vaultXPdaAccount = await program.account.vault.fetch(vaultXPda);

    expect(whitelistTokenAccount.tokens.find((key) => key.equals(mintX))).to.not
      .be.undefined;
    expect(whitelistTokenAccount.supportedTokenNum).to.equal(1);
    expect(whitelistTokenAccount.bump).to.equal(whitelistedTokenBump);
    expect(vaultXPdaAccount.mint.toBase58()).to.equal(mintX.toBase58());
    expect(vaultXPdaAccount.tokenAccount.toBase58()).to.equal(
      vaultXAta.toBase58()
    );
  });
  it("User can't whitelist the token X", async () => {
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

      expect.fail("User was trying to whitelist the token Y");
    } catch (err) {
      expect(err.error.errorMessage).to.equal(
        "Unauthorized access: this account is not the owner or authorized authority."
      );
    }
  });

  it("Onboards the User", async () => {
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

    console.log("Onboard User's transaction signature", tx);

    const userAccountInfo = await program.account.userAccount.fetch(
      userAccountPda
    );
    const configAccount = await program.account.config.fetch(configPda);

    expect(userAccountInfo.totalAmountStaked.toNumber()).to.equal(0);
    expect(userAccountInfo.totalYield.toNumber()).to.equal(0);
    expect(userAccountInfo.owner.toBase58()).to.equal(
      user.publicKey.toBase58()
    );
    expect(configAccount.totalUsers.toNumber()).to.equal(1);
  });
  it("Onboards the Merchant", async () => {
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

    console.log("Onboard Merchant's transaction signature", tx);

    const merchantAccountInfo = await program.account.merchantAccount.fetch(
      merchantAccountPda
    );
    const configAccount = await program.account.config.fetch(configPda);

    expect(merchantAccountInfo.businessName).to.equal(busineesNameA);
    expect(merchantAccountInfo.totalReceived.toNumber()).to.equal(0);
    expect(merchantAccountInfo.owner.toBase58()).to.equal(
      merchant.publicKey.toBase58()
    );
    expect(configAccount.totalMerchants.toNumber()).to.equal(1);
  });

  it("user stakes 10 tokens", async () => {

    const userXAtaInfoBefore=await getAccount(connection,userXAta);
    const vaultXAtaInfoBefore=await getAccount(connection,vaultXAta);

    console.log(`Before staking userXAta balance: ${userXAtaInfoBefore.amount}`);
    console.log(`Before staking vaultXAta balance: ${vaultXAtaInfoBefore.amount}`);

    const tx = await program.methods
      .stake(new anchor.BN(10))
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
    console.log("User stake's transaction signature", tx);

    const userStakeXaccount = await program.account.stakeAccount.fetch(
      stakeXAccount
    );

     const userXAtaInfoAfter=await getAccount(connection,userXAta);
    const vaultXAtaInfoAfter=await getAccount(connection,vaultXAta);

    console.log(`After staking userXAta balance: ${userXAtaInfoAfter.amount}`);
    console.log(`After staking vaultXAta balance: ${vaultXAtaInfoAfter.amount}`);

    assert.ok(vaultXAtaInfoBefore.amount<vaultXAtaInfoAfter.amount,"Vault X balance should increase");
    expect(userStakeXaccount.amountStaked.toNumber()).to.equals(10);
    expect(userStakeXaccount.isActive).to.equals(true);
    expect(userStakeXaccount.owner.toBase58()).to.equals(
      userAccountPda.toBase58()
    );
    expect(userStakeXaccount.mint.toBase58()).to.equals(mintX.toBase58());
  });

  // it("user stakes 3 tokens less than min deposit ", async () => {
  //   try {
  //     const tx = await program.methods
  //       .stake(new anchor.BN(10))
  //       .accountsStrict({
  //         user: user.publicKey,
  //         mintX: mintX,
  //         userAccount: userAccountPda,
  //         userXAta: userXAta,
  //         config: configPda,
  //         stakeAccount: stakeXAccount,
  //         whitelistedTokens: whitelistedTokenPda,
  //         vaultX: vaultXPda,
  //         vaultXAta: vaultXAta,
  //         yieldMint: yieldMint,
  //         yieldMintUserAta: yieldMintUserAta,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([user])
  //       .rpc();
  //     expect.fail(
  //       `User was staking 3 token less than min deposit ${min_deposit}`
  //     );
  //   } catch (err) {
  //     console.log("error: 428: ", err.error);
  //   }
  // });

  //   it("user stakes 10 again tokens", async () => {
  //   const tx = await program.methods
  //     .stake(new anchor.BN(10))
  //     .accountsStrict({
  //       user: user.publicKey,
  //       mintX: mintX,
  //       userAccount: userAccountPda,
  //       userXAta: userXAta,
  //       config: configPda,
  //       stakeAccount: stakeXAccount,
  //       whitelistedTokens: whitelistedTokenPda,
  //       vaultX: vaultXPda,
  //       vaultXAta: vaultXAta,
  //       yieldMint: yieldMint,
  //       yieldMintUserAta: yieldMintUserAta,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([user])
  //     .rpc();
      
  //     const userStakeXaccount = await program.account.stakeAccount.fetch(
  //       stakeXAccount
  //     );
  //     console.log("User stake's transaction signature", tx,userStakeXaccount.totalYield,userStakeXaccount.lastYieldMint);

  //   expect(userStakeXaccount.amountStaked.toNumber()).to.equals(20);
  //   expect(userStakeXaccount.isActive).to.equals(true);
  //   expect(userStakeXaccount.owner.toBase58()).to.equals(
  //     userAccountPda.toBase58()
  //   );
  //   expect(userStakeXaccount.mint.toBase58()).to.equals(mintX.toBase58()); 
  //   // expect(userStakeXaccount.totalYield.toNumber()).to.not.equals(0);  
  // });

});
