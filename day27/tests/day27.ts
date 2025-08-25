import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day27 } from "../target/types/day27";

describe("day27", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day27 as Program<Day27>;

  it("Is initialized!", async () => {
    const [myPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    )
    await program.methods.initialize()
    .accounts({
      myPda: myPda
    })
    .rpc();

    await program.methods.giveToSystemProgram()
    .accounts({
      myPda: myPda
    })
    .rpc()
    await program.methods.initialize()
    .accounts({
      myPda: myPda
    })
    .rpc();
    console.log("Account re inite after giving to system program");

    await program.methods.drainLamports()
    .accounts({
      myPda: myPda
    })
    .rpc();
  
    await program.methods.initialize()
    .accounts({
      myPda: myPda
    })
    .rpc();
    console.log("Account re inite after draining lamports !");

    await program.methods.erase()
    .accounts({
      myPda: myPda
    })
    .rpc();

    try{
      await program.methods.initialize()
      .accounts({
        myPda: myPda
      })
      .rpc();
      console.log("Account re inite after draining lamports !");
    } catch {
      console.log("Failed to init after Erase because account still own by program");
    }

  });
});
