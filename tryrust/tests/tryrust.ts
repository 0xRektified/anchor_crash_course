import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tryrust } from "../target/types/tryrust";

describe("tryrust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tryrust as Program<Tryrust>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is ageChecker!", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(18).rpc();
    console.log("Your transaction signature", tx);
  });


  it("Is ageChecker!", async () => {
    // Add your test here.
    const tx = await program.methods.ageCheckerMatch(2).rpc();
    console.log("Your transaction signature", tx);

    const tx2 = await program.methods.ageCheckerMatch(5).rpc();
    console.log("Your transaction signature", tx2);

    const tx3 = await program.methods.ageCheckerMatch(9).rpc();
    console.log("Your transaction signature", tx3);
  });

  it("Is loop!", async () => {
    // Add your test here.
    const tx = await program.methods.fnLoop().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is hashmap!", async () => {
    // Add your test here.
    const tx = await program.methods.hashMap(
      "name",
      "bob"
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is Struct!", async () => {
    // Add your test here.
    const tx = await program.methods.fnStruct(
      "name",
      new anchor.BN(25)
    ).rpc();
    console.log("Your transaction signature", tx);
  });


  it("Is exercise", async () => {
    let test = [
      new anchor.BN(3),
      new anchor.BN(2),
      new anchor.BN(8)
    ]
    // Add your test here.
    const tx = await program.methods.exercise(test).rpc();
    console.log("Your transaction signature", tx);
  });
});
