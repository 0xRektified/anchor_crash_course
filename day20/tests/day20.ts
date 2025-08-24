import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day20 } from "../target/types/day20";

describe("day20", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day20 as Program<Day20>;
  let pubkey = new anchor.web3.PublicKey("azAq71cZVjpiQvd7c4qAicrq1zQNM8Eb84xR8Y81w5o");
  it("Is initialized!", async () => {
    const [Value] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    )

    console.log(`Account Id: ${Value}`);

    await program.methods.initialize()
    .accounts({
      val: Value
    })
    .rpc();

    // not necessary as it ll be auto infer with seeds=[]
    // deterministic
    await program.methods.increaseAccountSize()
    .accounts({
      val: Value
    })
    .rpc();

    // We use UncheckedAccount to check the user account here
    await program.methods.readBalance()
    .accounts({
      val: pubkey
    })
    .rpc();
  });

});
