import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day26 } from "../target/types/day26";

describe("day26", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day26 as Program<Day26>;

  it("Is initialized!", async () => {
    const programId = await program.account.myPda.programId
    const seeds = []
    let pdaAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      programId
    )[0]

    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      pda: pdaAccount
    })
    .rpc();
    console.log("Your transaction signature", tx);

    await program.methods.donate(
      new anchor.BN(100e9)
    )
    .accounts({
      pda: pdaAccount
    })
    .rpc();

    console.log(
      "lamport balance of ",
      await anchor.getProvider().connection.getBalance(pdaAccount)
    );

    await program.methods.withdraw(
      new anchor.BN(10e9)
    )
    .accounts({
      pda: pdaAccount
    })
    .rpc();

    console.log(
      "lamport balance of ",
      await anchor.getProvider().connection.getBalance(pdaAccount)
    );
  });
});
