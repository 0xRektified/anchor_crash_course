import * as anchor from "@coral-xyz/anchor";
import { BorshCoder, EventParser, Program } from "@coral-xyz/anchor";
import { Day13 } from "../target/types/day13";

describe("day13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day13 as Program<Day13>;

  it("Is initialized!", async () => {
    // Execute transaction
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    // Wait for confirmation
    const latestBlockhash = await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction({
      signature: tx,
      ...latestBlockhash,
    });
    
    // Fetch and parse events from the transaction
    const txDetails = await program.provider.connection.getTransaction(tx, {
      maxSupportedTransactionVersion: 0,
      commitment: "confirmed"
    });
    
    // Parse events from transaction logs
    const eventParser = new EventParser(program.programId, program.coder);
    const events = eventParser.parseLogs(txDetails.meta.logMessages);
    
    for (const event of events) {
      console.log("Event name:", event.name);
      console.log("Event data:", event.data);
    }
  });
});
