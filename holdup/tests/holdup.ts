import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { Holdup } from "../target/types/holdup";

interface User {
  signer: Keypair;
  collateralAta: PublicKey;
  lentAta: PublicKey;
  position: PublicKey;
}

const tokenProgram = TOKEN_2022_PROGRAM_ID;

describe("holdup", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.holdup as Program<Holdup>;
  const connection = anchor.getProvider().connection;

  it("Initializes and increments a counter", async () => {
    const aliceKey = Keypair.generate();
    const bobKey = Keypair.generate();
    const mintAuthority = Keypair.generate();

    await Promise.all([
      connection.requestAirdrop(aliceKey.publicKey, 1e9),
      connection.requestAirdrop(bobKey.publicKey, 1e9),
    ]);

    const lentMint = await createMint(
      connection,
      aliceKey,
      mintAuthority.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    const collateralMint = await createMint(
      connection,
      aliceKey,
      mintAuthority.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    const collateralTokenAtaAlice = await getOrCreateAssociatedTokenAccount(
      connection,
      aliceKey,
      lentMint,
      aliceKey.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    const lentTokenAtaAlice = await getOrCreateAssociatedTokenAccount(
      connection,
      aliceKey,
      collateralMint,
      aliceKey.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await mintTo(
      connection,
      aliceKey,
      lentMint,
      collateralTokenAtaAlice.address,
      mintAuthority,
      1000000,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    const lentTokenAtaBob = await getOrCreateAssociatedTokenAccount(
      connection,
      bobKey,
      lentMint,
      bobKey.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    const collateralTokenAtaBob = await getOrCreateAssociatedTokenAccount(
      connection,
      bobKey,
      collateralMint,
      bobKey.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await mintTo(
      connection,
      bobKey,
      collateralMint,
      collateralTokenAtaBob.address,
      mintAuthority,
      1000000,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await balanceEquals(connection, aliceKey, lentMint, 1000000);
    await balanceEquals(connection, bobKey, collateralMint, 1000000);

    const [counterAlice] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), aliceKey.publicKey.toBuffer()],
      program.programId,
    );
    const [counterBob] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), bobKey.publicKey.toBuffer()],
      program.programId,
    );
    const [pool] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), lentMint.toBuffer(), collateralMint.toBuffer()],
      program.programId,
    );

    const alice: User = {
      signer: aliceKey,
      collateralAta: collateralTokenAtaAlice.address,
      lentAta: lentTokenAtaAlice.address,
      position: counterAlice,
    };

    const bob: User = {
      signer: bobKey,
      collateralAta: collateralTokenAtaBob.address,
      lentAta: lentTokenAtaBob.address,
      position: counterBob,
    };

    const accountBeforeCreation = await program.account.position.fetchNullable(
      counterAlice,
    );
    assert.equal(accountBeforeCreation, null);

    await program.methods
      .initialize()
      .accountsPartial({
        lentMint,
        collateralMint,
        tokenProgram,
        priceAuthority: mintAuthority.publicKey,
        pool,
      })
      .rpc();

    await program.methods
      .register()
      .accountsPartial({
        position: alice.position,
        payer: alice.signer.publicKey,
        collateralMint,
        tokenProgram,
        pool,
      })
      .signers([alice.signer])
      .rpc();
    await program.methods
      .register()
      .accountsPartial({
        position: bob.position,
        payer: bob.signer.publicKey,
        collateralMint,
        tokenProgram,
        pool,
      })
      .signers([bob.signer])
      .rpc();

    const accountAfterCreation = await program.account.position.fetchNullable(
      counterAlice,
    );
    assert.ok(accountAfterCreation?.deposited.eq(new anchor.BN(0)));

    const aliceDepositAmount = 100000;
    await program.methods
      .depositLent(new anchor.BN(aliceDepositAmount))
      .accountsPartial({
        position: counterAlice,
        payerAtaLent: alice.collateralAta,
        payer: aliceKey.publicKey,
        lentMint,
        tokenProgram,
        pool,
      })
      .signers([aliceKey])
      .rpc();

    await balanceEquals(
      connection,
      aliceKey,
      lentMint,
      1000000 - aliceDepositAmount,
    );
    await positionEquals(program, counterAlice, aliceDepositAmount, 0, 0);

    await positionEquals(program, bob.position, 0, 0, 0);

    const bobCollateralDepositAmount = 1000;
    await program.methods
      .depositCollateral(new anchor.BN(bobCollateralDepositAmount))
      .accountsPartial({
        position: bob.position,
        payerAtaCollateral: bob.collateralAta,
        payer: bob.signer.publicKey,
        collateralMint,
        tokenProgram,
        pool,
      })
      .signers([bob.signer])
      .rpc();

    await positionEquals(
      program,
      bob.position,
      0,
      0,
      bobCollateralDepositAmount,
    );

    await program.methods
      .updatePrice(10 * 10000)
      .accountsPartial({
        pool,
        priceAuthority: mintAuthority.publicKey,
      })
      .signers([mintAuthority])
      .rpc();

    const bobBorrowAmount = 5000;
    await program.methods
      .borrow(new anchor.BN(bobBorrowAmount))
      .accountsPartial({
        position: bob.position,
        payerAtaLent: bob.lentAta,
        payer: bob.signer.publicKey,
        lentMint,
        tokenProgram,
        pool,
      })
      .signers([bobKey])
      .rpc();

    await positionEquals(
      program,
      bob.position,
      0,
      bobBorrowAmount,
      bobCollateralDepositAmount,
    );
  });
});

async function balanceEquals(
  connection: Connection,
  signer: Keypair,
  mint: PublicKey,
  amount: number,
) {
  const tokenAtaAfterMint = await getOrCreateAssociatedTokenAccount(
    connection,
    signer,
    mint,
    signer.publicKey,
    undefined,
    undefined,
    undefined,
    TOKEN_2022_PROGRAM_ID,
  );
  assert.equal(tokenAtaAfterMint.amount, BigInt(amount));
}

async function positionEquals(
  program: Program<Holdup>,
  address: PublicKey,
  deposited: number,
  borrowed: number,
  collateral: number,
) {
  const account = await program.account.position.fetch(address);
  assert.equal(
    account.deposited.toString(),
    new anchor.BN(deposited).toString(),
  );
  assert.equal(account.borrowed.toString(), new anchor.BN(borrowed).toString());

  assert.equal(
    account.collateral.toString(),
    new anchor.BN(collateral).toString(),
  );
}
