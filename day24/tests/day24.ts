import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day24 } from "../target/types/day24";

async function airdropSol(publicKey, amount){
  let airdropTx = 
    await anchor.getProvider().connection.requestAirdrop(publicKey, amount);
    await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx){
  const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  })
}

describe("day24", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day24 as Program<Day24>;

  it("Is initialized!", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();
    await airdropSol(alice.publicKey, 1e9);
    await airdropSol(bob.publicKey, 1e9);

    let seeds = [];
    const [store, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId,
    );
    await program.methods.initialize()
    .accounts({
      store: store,
      fren: alice.publicKey
    })
    .signers([alice])
    .rpc();

    await program.methods.updateValue(
      new anchor.BN(3)
    )
    .accounts({
      store: store,
      fren: bob.publicKey
    })
    .signers([bob])
    .rpc();
  });


  it("Is test point transfer!", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();
    await airdropSol(alice.publicKey, 1e9);
    await airdropSol(bob.publicKey, 1e9);

    let seeds_alice = [alice.publicKey.toBytes()];
    const [PlayerAlice, _bumpAlice] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_alice,
      program.programId,
    );

    let seeds_bob = [bob.publicKey.toBytes()];
    const [PlayerBob, _bumpBob] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_bob,
      program.programId,
    );

    await program.methods.setupGame().accounts({
      player: PlayerAlice,
      signer: alice.publicKey
    })
    .signers([alice])
    .rpc();

    await program.methods.setupGame().accounts({
      player: PlayerBob,
      signer: bob.publicKey
    })
    .signers([bob])
    .rpc();

    await program.methods.transferPoints(5).accounts({
      from: PlayerAlice,
      to: PlayerBob,
      authority: alice.publicKey
    }).signers([alice]).rpc();

    console.log(`Alice has ${(await program.account.player.fetch(PlayerAlice)).points} points`);
    console.log(`Bob has ${(await program.account.player.fetch(PlayerBob)).points} points`)

    // Will error out because of the constraints

    await program.methods.transferPoints(5).accounts({
      from: PlayerAlice,
      to: PlayerBob,
      authority: alice.publicKey
    }).signers([alice]).rpc();
  })

  it("It should fail to call the tranfer point with wrong signature!", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();
    await airdropSol(alice.publicKey, 1e9);
    await airdropSol(bob.publicKey, 1e9);

    let seeds_alice = [alice.publicKey.toBytes()];
    const [PlayerAlice, _bumpAlice] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_alice,
      program.programId,
    );

    let seeds_bob = [bob.publicKey.toBytes()];
    const [PlayerBob, _bumpBob] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_bob,
      program.programId,
    );
    await program.methods.setupGame().accounts({
      player: PlayerAlice,
      signer: alice.publicKey
    })
    .signers([alice])
    .rpc();
    
    await program.methods.setupGame().accounts({
      player: PlayerBob,
      signer: bob.publicKey
    })
    .signers([bob])
    .rpc();
    
    try {
      await program.methods.transferPoints(5).accounts({
        from: PlayerAlice,
        to: PlayerBob,
        authority: bob.publicKey
      }).signers([bob]).rpc();
      throw new Error("Expected transaction to fail but it succeeded");
    } catch (error) {
      console.log("Transaction failed as expected with wrong authority");
    }
  })

  it("It should fail to call the tranfer point more point that owned!", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();
    await airdropSol(alice.publicKey, 1e9);
    await airdropSol(bob.publicKey, 1e9);

    let seeds_alice = [alice.publicKey.toBytes()];
    const [PlayerAlice, _bumpAlice] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_alice,
      program.programId,
    );
    let seeds_bob = [bob.publicKey.toBytes()];
    const [PlayerBob, _bumpBob] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_bob,
      program.programId,
    );
    await program.methods.setupGame().accounts({
      player: PlayerAlice,
      signer: alice.publicKey
    })
    .signers([alice])
    .rpc();
    
    await program.methods.setupGame().accounts({
      player: PlayerBob,
      signer: bob.publicKey
    })
    .signers([bob])
    .rpc();
    
    try {
      await program.methods.transferPoints(25).accounts({
        from: PlayerAlice,
        to: PlayerBob,
        authority: alice.publicKey
      }).signers([alice]).rpc();
      throw new Error("Expected transaction to fail but it succeeded");
    } catch (error) {
      console.log("Transaction failed as expected with insufficient points");
    }
  })
});
