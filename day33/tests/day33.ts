import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day33 } from "../target/types/day33";
import { Alice } from "../target/types/alice";
import { expect } from "chai";

describe("day33", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.day33 as Program<Day33>;
  const alice_program = anchor.workspace.Alice as Program<Alice>;
  const [Pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [], // seeds used in the program
    program.programId
  );

  it("Is initialized!", async () => {

    await program.methods.initialize()
    .accounts({
      day33DataAccount: Pda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  });


  it("Can add number then double!", async () => {
    const tx = await alice_program.methods.askDay33ToAdd(
      new anchor.BN(10),
      new anchor.BN(20),
    )
    .accounts({
      day33DataAccount: Pda,
      day33Program: program.programId,
    })
    .rpc();
  })

  it("Can assert value in day33's data account equals 4 + 2", async () => {

    const day33Value = (
      await program.account.store.fetch(Pda)
    ).result.toNumber();
    expect(day33Value).to.equal(30);
  });
});
