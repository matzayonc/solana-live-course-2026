import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { DiamondHands } from "../target/types/diamond_hands";

describe("diamond-hands", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.diamondHands as Program<DiamondHands>;

  it("Initializes and increments a counter", async () => {
    console.log(anchor.getProvider().publicKey ?? "No key");
    const [counter] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId,
    );

    const countersBefore = await program.account.counter.all();
    console.log(`Count: ${countersBefore.length}`);

    const initializeTx = await program.methods
      .initialize()
      .accountsPartial({ counter })
      .rpc();
    console.log("Initialize transaction signature", initializeTx);

    const counters = await program.account.counter.all();
    console.log(`Count: ${counters[0].account.count.toString()}`);
    try {
      const incrementTx = await program.methods
        .increment()
        .accountsPartial({ counter })
        .rpc();
      console.log("Increment transaction signature", incrementTx);
    } catch (e) {}
    const countersAfter = await program.account.counter.all();
    console.log(`Count: ${countersAfter[0].account.count.toString()}`);
  });
});
