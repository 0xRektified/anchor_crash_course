import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day14";

describe("day14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day14 as Program<Day14>;

  let myKeypair = anchor.web3.Keypair.generate();
  let myKeypair2 = anchor.web3.Keypair.generate();

  let KeypairAttack = anchor.web3.Keypair.generate();
  it("Is signed by a single signer!", async () => {
    // Add your test here.
    await program.methods.initialize()
    .accounts({
      signer1: program.provider.publicKey,
    })
    .rpc();
    console.log("The signer1: ", program.provider.publicKey.toBase58());
  });

  it("Is signed by multi signers!", async () => {
    // Add your test here.
    await program.methods.initialize()
    .accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
      signer3: myKeypair2.publicKey,
    })
    .signers([myKeypair, myKeypair2])
    .rpc();
    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());
    console.log("The signer3: ", myKeypair2.publicKey.toBase58());
  });

  it("Is Owner!", async () => {
    // Add your test here.
    await program.methods.access()
    .accounts({
      signerAccount: program.provider.publicKey,
    })
    .rpc();
  });

    it("Is NotOwner!", async () => {
    // Add your test here.
    await program.methods.access()
    .accounts({
      signerAccount: KeypairAttack.publicKey,
    })
    .signers([KeypairAttack])
    .rpc();
  });
});
