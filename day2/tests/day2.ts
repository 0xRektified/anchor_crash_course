import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(
      new anchor.BN(777),
      new anchor.BN(888),
      "hello"
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async() =>{
    const tx = await program.methods.array(
      [
        new anchor.BN(1),
        new anchor.BN(2),
      ]
    ).rpc();
    console.log("The array", tx );
  })

  it("test overflow", async() => {
    const tx = await program.methods.testOverflow(
      new anchor.BN(0),
      new anchor.BN(1),
    ).rpc();
    console.log("Overflow", tx );
  })

  it("test math", async() => {
    const tx = await program.methods.mathOp(
      new anchor.BN(2),
      8,
    ).rpc();
    console.log("test math", tx );
  })
  it("test calc", async() => {
    const tx = await program.methods.calc(
      new anchor.BN(6),
      new anchor.BN(2),
      "+"
    ).rpc();
    await program.methods.calc(
      new anchor.BN(6),
      new anchor.BN(0),
      "sqrt"
    ).rpc();

    await program.methods.calc(
      new anchor.BN(11),
      new anchor.BN(0),
      "log10"
    ).rpc();
    console.log("test math", tx );
  })
});
