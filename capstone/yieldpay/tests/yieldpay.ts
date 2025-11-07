import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Yieldpay } from "../target/types/yieldpay";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("yieldpay", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 const connection=provider.connection;
  const program = anchor.workspace.yieldpay as Program<Yieldpay>;

  //TODO:  define everthing firstly -----------

  const admin = provider.wallet;
  const user = anchor.web3.Keypair.generate();
  const merchant = anchor.web3.Keypair.generate();

  let userAccount: anchor.web3.PublicKey;
  let merchantAccount: anchor.web3.PublicKey;
  let configPda: anchor.web3.PublicKey;
  let configBump: number;
  let yieldMint: anchor.web3.PublicKey;
  let yieldMintBump: number;
  let whitelistedTokenPda: anchor.web3.PublicKey;
  let whitelistedTokenBump: number;
  let vaultXPda: anchor.web3.PublicKey;
  let vaultXBump: number;
  let vaultXAta: anchor.web3.PublicKey;
  let mintX: anchor.web3.PublicKey;
  let vaultYPda: anchor.web3.PublicKey;
  let vaultYBump: number;
  let vaultYAta: anchor.web3.PublicKey;
  let mintY: anchor.web3.PublicKey;
  let max_stake = new anchor.BN(10);
  let min_deposit = 1;
  let total_users = new anchor.BN(0);
  let total_merchants = new anchor.BN(0);
  let yield_min_period = new anchor.BN(2); //days
  let apy_bps = 10;

  const CONFIG_SEED = "CONFIG";
  const YIELD_MINT_SEED = "YIELD";
  const WHITELIST_SEED = "WHITELIST";
  const VAULT_SEED = "VAULT";

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

    [yieldMint, yieldMintBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(YIELD_MINT_SEED), configPda.toBuffer()],
      program.programId
    );

    console.log(
      `yieldMint: ${yieldMint.toString()} and yieldMintBump: ${yieldMintBump}`
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

    console.log("mintX: ",mintX.toString());

    [vaultXPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED), mintX.toBuffer(), configPda.toBuffer()],
      program.programId
    );

    vaultXAta = getAssociatedTokenAddressSync(mintX, vaultXPda, true);

    console.log(
      `vaultPda: ${vaultXPda.toString()} , vaultBump: ${vaultBump} and its associated vaultX: ${vaultXAta.toString()}`
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
    expect(configAccount.apyBps).to.equal(apy_bps);
    expect(configAccount.totalUsers.toString()).to.equal(
      total_users.toString()
    );
    expect(configAccount.totalMerchants.toString()).to.equal(
      total_merchants.toString()
    );
    expect(configAccount.maxStake.toString()).to.equal(max_stake.toString());
    expect(configAccount.minDeposit).to.equal(min_deposit);
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
    const vaultXPdaAccount= await program.account.vault.fetch(vaultXPda);

    //todo: check vault state too ----
    expect(whitelistTokenAccount.tokens.find((key)=>key.equals(mintX))).to.not.be.undefined;
    expect(whitelistTokenAccount.supportedTokenNum).to.equal(1);
    expect(whitelistTokenAccount.bump).to.equal(whitelistedTokenBump);
    expect(vaultXPdaAccount.mint.toBase58()).to.equal(mintX.toBase58());
    expect(vaultXPdaAccount.tokenAccount.toBase58()).to.equal(vaultXAta.toBase58());
  });
  it("User can't whitelist the token X", async () => {
    try {
      const tx = await program.methods
        .whitelistToken()
        .accountsStrict({
          admin: user.publicKey,
          mintX: mintX,
          whitelistedTokens: whitelistedTokenPda,
          vaultX: vaultXPda,
          vaultXAta: vaultXAta,
          config: configPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();

      expect.fail("User was trying to whitelist the token X");
    } catch (err) {
      // expect(err.error.errorMessage).to.equal("UnauthorizedAccess");
      console.log("207...", err?.toString());
    }
  });

  // it("Onboards the User", async () => {});
  // it("Onboards the Merchant", async () => {});
});
