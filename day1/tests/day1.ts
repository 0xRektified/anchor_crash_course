import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day1 } from "../target/types/day1";

describe("day1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day1 as Program<Day1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize2().rpc();
    console.log("Your transaction signature", tx);
  });
});
