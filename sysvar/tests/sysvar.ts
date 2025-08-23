import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.sysvar as Program<Sysvar>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Get day of the week!", async () => {
    // Add your test here.
    const tx = await program.methods.getDayOfWeek().rpc();
    console.log("Your transaction signature", tx);
  });
});
