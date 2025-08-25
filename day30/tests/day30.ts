import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day30 } from "../target/types/day30";

describe("day30", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day30 as Program<Day30>;

  it("Is initialized!", async () => {
    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId,
    )
    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      pda: pda
    })
    .rpc();
    console.log("Your transaction signature", tx);

    await program.methods.delete()
    .accounts({
      pda: pda
    })
    .rpc();

    let account = await program.account.myPda.fetchNullable(pda);
    console.log(account);

    // we can init again after closing
    await program.methods.initialize()
    .accounts({
      pda: pda
    })
    .rpc();

    const wallet = anchor.web3.Keypair.generate();
    try {
      await program.methods.delete()
      .accounts({
        pda: pda,
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
    } catch {
      console.log(`${wallet.publicKey} not allowed to delete only authority is`)
    }
  });
});

// We can also close a program, but it's definitive
// solana program close <address> --bypass warning
