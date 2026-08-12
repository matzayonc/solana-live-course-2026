import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { BN } from "bn.js";
import { DiamondHands } from "../target/types/diamond_hands";

const MINT_AMOUNT = new BN(200000000);
const LOCKUP_AMOUNT = new BN(100000000);

describe("diamond-hands", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const payer = anchor.getProvider().wallet?.payer!;
  const connection = anchor.getProvider().connection;

  const program = anchor.workspace.diamondHands as Program<DiamondHands>;

  it("Initializes and increments a counter", async () => {
    const mint = await createMint(
      connection,
      payer,
      payer.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    const payerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      payer.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    const providerKey = anchor.getProvider().publicKey!;
    const [locker] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("locker"), providerKey.toBuffer()],
      program.programId,
    );

    const mentor = anchor.web3.Keypair.generate();

    const testData: TestData = {
      program,
      locker,
      connection,
      payer,
      mint,
    };

    await check(testData, new BN(0), null, "fresh");

    await mintTo(
      connection,
      payer,
      mint,
      payerAta.address,
      payer,
      MINT_AMOUNT.toNumber(),
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await check(testData, new BN(MINT_AMOUNT), null, "after mint");

    await program.methods
      .lockup(LOCKUP_AMOUNT)
      .accounts({
        payer: payer.publicKey,
        authority: mentor.publicKey,
        payerAta: payerAta.address,
        mint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();

    await check(
      testData,
      new BN(MINT_AMOUNT.sub(LOCKUP_AMOUNT)),
      new BN(LOCKUP_AMOUNT),
      "after lockup",
    );

    console.log("before wait");
    await new Promise((r) => setTimeout(r, 5000));
    // Do something to advance block
    console.log("after wait");

    await program.methods
      .withdraw()
      .accountsPartial({
        locker,
        payerAta: payerAta.address,
        authority: mentor.publicKey,
        mint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([mentor])
      .rpc();

    await check(testData, new BN(199950000), new BN(0), "after withdraw");
  });
});

interface TestData {
  program: Program<DiamondHands>;
  locker: anchor.web3.PublicKey;
  connection: anchor.web3.Connection;
  payer: anchor.web3.Signer;
  mint: anchor.web3.PublicKey;
}

async function check(
  { program, locker, connection, payer, mint }: TestData,
  unlocked: anchor.BN,
  locked: anchor.BN | null,
  context: string,
) {
  const lockup = await program.account.locker.fetchNullable(locker);

  const payerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey,
    false,
    undefined,
    undefined,
    TOKEN_2022_PROGRAM_ID,
  );

  if (locked != null && lockup != null) {
    assertEqual(
      new BN(lockup.amount),
      locked,
      `locked amount: ${lockup.amount} is not the expected ${locked} at ${context}`,
    );
  } else if (locked == null && lockup == null) {
  } else if (locked == null) throw "expected null";
  else if (lockup == null) throw "account null";

  const payerBalance = payerAta.amount;
  assertEqual(
    new BN(payerBalance),
    unlocked,
    `balance: ${payerBalance} is not the expected ${unlocked} at ${context}`,
  );
}

function assertEqual(left: anchor.BN, right: anchor.BN, context: string) {
  if (!left.eq(right)) {
    throw `${context}`;
  }
}
