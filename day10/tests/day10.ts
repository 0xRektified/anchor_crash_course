import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day10 } from "../target/types/day10";

describe("day10", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day10 as Program<Day10>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


  it("add two numbers!", async () => {
    // Add your test here.
    const tx = await program.methods.addTwoNumber(
      new anchor.BN(2),
      new anchor.BN(4),
    ).rpc();
    console.log("Your transaction signature", tx);
  });
});
