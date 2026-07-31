import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  mintTo,
} from "@solana/spl-token";
import { expect } from "chai";
import BN from "bn.js";
import type { DiamondHands } from "../target/types/diamond_hands.ts";

// `BN` is not an ESM named export of `@anchor-lang/core` — it lives only on the
// CJS default export — so it is imported from its own package instead.
const { Keypair, PublicKey, LAMPORTS_PER_SOL, SYSVAR_CLOCK_PUBKEY } =
  anchor.web3;

const DECIMALS = 6;
const DEPOSIT = 1_000_000n; // 1 token
const LOCK_SEED = Buffer.from("lock");
const VAULT_SEED = Buffer.from("vault");

/// Clock sysvar layout: slot, epoch_start_timestamp, epoch,
/// leader_schedule_epoch, then unix_timestamp — five 8-byte fields.
const CLOCK_UNIX_TIMESTAMP_OFFSET = 32;

describe("diamond-hands", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.diamondHands as Program<DiamondHands>;

  const owner = Keypair.generate();

  let mint: anchor.web3.PublicKey;
  let ownerTokenAccount: anchor.web3.PublicKey;
  let lock: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;

  before(async () => {
    const airdrop = await provider.connection.requestAirdrop(
      owner.publicKey,
      2 * LAMPORTS_PER_SOL,
    );
    await confirm(airdrop);

    // The payer of the mint is irrelevant to the program; reuse the provider
    // wallet so the test does not need a second funded account.
    mint = await createMint(
      provider.connection,
      owner,
      owner.publicKey,
      null,
      DECIMALS,
    );

    ownerTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      owner,
      mint,
      owner.publicKey,
    );

    await mintTo(
      provider.connection,
      owner,
      mint,
      ownerTokenAccount,
      owner,
      DEPOSIT,
    );

    [lock] = PublicKey.findProgramAddressSync(
      [LOCK_SEED, owner.publicKey.toBuffer(), mint.toBuffer()],
      program.programId,
    );
    [vault] = PublicKey.findProgramAddressSync(
      [VAULT_SEED, lock.toBuffer()],
      program.programId,
    );
  });

  it("rejects a zero amount", async () => {
    await expectError(
      program.methods
        .lock(new BN(0), new BN(60))
        .accountsPartial(accounts())
        .signers([owner])
        .rpc(),
      "ZeroAmount",
    );
  });

  it("locks tokens into the vault", async () => {
    // Short enough that the withdraw test can simply wait it out.
    await program.methods
      .lock(new BN(DEPOSIT.toString()), new BN(2))
      .accountsPartial(accounts())
      .signers([owner])
      .rpc();

    const account = await program.account.lock.fetch(lock);
    expect(account.owner.toBase58()).to.equal(owner.publicKey.toBase58());
    expect(account.mint.toBase58()).to.equal(mint.toBase58());
    expect(BigInt(account.amount.toString())).to.equal(DEPOSIT);

    expect((await getAccount(provider.connection, vault)).amount).to.equal(
      DEPOSIT,
    );
    expect(
      (await getAccount(provider.connection, ownerTokenAccount)).amount,
    ).to.equal(0n);
  });

  it("refuses to withdraw before the lock expires", async () => {
    await expectError(
      program.methods
        .withdraw()
        .accountsPartial(accounts())
        .signers([owner])
        .rpc(),
      "StillLocked",
    );
  });

  it("withdraws once the lock expires", async () => {
    const { unlocksAt } = await program.account.lock.fetch(lock);
    await timeTravelTo(Number(unlocksAt) + 1);
    expect(await chainTime()).to.be.at.least(Number(unlocksAt));

    await program.methods
      .withdraw()
      .accountsPartial(accounts())
      .signers([owner])
      .rpc();

    expect(
      (await getAccount(provider.connection, ownerTokenAccount)).amount,
    ).to.equal(DEPOSIT);

    // Both PDAs are closed and their rent refunded.
    expect(await provider.connection.getAccountInfo(lock)).to.be.null;
    expect(await provider.connection.getAccountInfo(vault)).to.be.null;
  });

  /// Every account the two instructions take. Passed explicitly rather than
  /// letting Anchor resolve them, so a failing test points at the program
  /// rather than at address inference.
  function accounts() {
    return {
      owner: owner.publicKey,
      mint,
      ownerTokenAccount,
      lock,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
    };
  }

  async function confirm(signature: string) {
    const latest = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature, ...latest });
  }

  /// The chain clock, not the wall clock, decides when a lock expires — so the
  /// test reads the same Clock sysvar the program does.
  async function chainTime() {
    const clock = await provider.connection.getAccountInfo(SYSVAR_CLOCK_PUBKEY);
    if (clock === null) throw new Error("clock sysvar is unreadable");
    return Number(clock.data.readBigInt64LE(CLOCK_UNIX_TIMESTAMP_OFFSET));
  }

  /// Jump the chain clock instead of waiting for it.
  ///
  /// `anchor test` runs on Surfpool with `--block-production-mode transaction`:
  /// blocks are only produced when a transaction arrives, so the clock is
  /// frozen while the test sleeps and no amount of waiting expires a lock.
  /// `surfnet_timeTravel` moves it directly. Note the parameter is in
  /// milliseconds while the Clock sysvar is in seconds.
  async function timeTravelTo(timestamp: number) {
    const response = await fetch(provider.connection.rpcEndpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "surfnet_timeTravel",
        params: [{ absoluteTimestamp: timestamp * 1000 }],
      }),
    });

    const { error } = (await response.json()) as { error?: { data?: string } };
    if (error) throw new Error(`surfnet_timeTravel failed: ${error.data}`);
  }

  async function expectError(promise: Promise<unknown>, code: string) {
    try {
      await promise;
    } catch (error) {
      expect((error as anchor.AnchorError).error.errorCode.code).to.equal(code);
      return;
    }
    throw new Error(`expected the instruction to fail with ${code}`);
  }
});
