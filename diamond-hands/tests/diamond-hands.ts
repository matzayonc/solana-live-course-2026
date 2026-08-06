import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { assert } from "chai";
import { DiamondHands } from "../target/types/diamond_hands";

describe("diamond-hands", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.diamondHands as Program<DiamondHands>;

  it("Initializes and increments a counter", async () => {
    const providerKey = anchor.getProvider().publicKey!;
    const [locker] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("locker"), providerKey.toBuffer()],
      program.programId,
    );

    await program.methods
      .initialize(new anchor.BN(2).pow(new anchor.BN(4)))
      .accountsPartial({ counter: locker })
      .rpc();
    const counters = await program.account.locker.all();
    await program.methods.increment().accountsPartial({ locker }).rpc();
    const countersAfter = await program.account.locker.all();

    // assert.equal(counters[0].account.amount.toString(), "40");
    assert.equal(countersAfter[0].account.amount.toString(), "0");
  });
});
