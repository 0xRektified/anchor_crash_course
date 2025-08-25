import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day25 } from "../target/types/day25";

async function airdropSol(publicKey, amount){
  let airdropTx = await anchor.getProvider().connection.requestAirdrop(
    publicKey, amount * anchor.web3.LAMPORTS_PER_SOL
  );
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  })
}

describe("day25", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day25 as Program<Day25>;

  it("Is initialized!", async () => {
    const seeds =[]

    const [MyPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )
 
    console.log("owner of pda before initialize:",
    await anchor.getProvider().connection.getAccountInfo(MyPDA));

    // Add your test here.
    const tx = await program.methods.initializePda()
    .accounts({
      storePda: MyPDA
    })
    .rpc();
    console.log("Initialise pda at", tx);
    console.log("owner of pda after initialize:",
    (await anchor.getProvider().connection.getAccountInfo(MyPDA)).owner.toBase58());
  


    const newKeyPair = anchor.web3.Keypair.generate();
    const newKeyPair2 = anchor.web3.Keypair.generate();
    
    const accountBefore = await anchor.getProvider().connection.getAccountInfo(newKeyPair.publicKey);
    console.log("Account before airdrop:", accountBefore);
    console.log("Owner before airdrop:", accountBefore?.owner?.toBase58() || "Account doesn't exist");

    await airdropSol(newKeyPair.publicKey, 1);
    
    const accountAfter = await anchor.getProvider().connection.getAccountInfo(newKeyPair.publicKey);
    console.log("Account after airdrop exists:", accountAfter !== null);
    console.log("Owner after airdrop:", accountAfter?.owner?.toBase58() || "Account doesn't exist");
    console.log("Lamports after airdrop:", accountAfter?.lamports);
      
    console.log(`new key pair is ${newKeyPair.publicKey.toBase58()}`);

    try {
      const tx3 = await program.methods.initializeKeyPairAccount()
      .accounts({
        storeKey: newKeyPair2.publicKey
      })
      .signers([newKeyPair])
      .rpc();
      console.log(`initialise newKeyPair2 at ${tx3}`);
    } catch {
      console.log(`This should fail because the store key is not the signer (we don't own the private key)`);

    }

    const tx2 = await program.methods.initializeKeyPairAccount()
    .accounts({
      storeKey: newKeyPair.publicKey
    })
    .signers([newKeyPair])
    .rpc();
    console.log(`initialise key at ${tx2}`);

    console.log("owner of keypair after initialize:",
      (await anchor.getProvider().connection.getAccountInfo(newKeyPair.publicKey)).owner.toBase58());
   
    try{
      const tx4 = await program.methods.initializeKeyPairAccount()
      .accounts({
        storeKey: MyPDA
      })
      .rpc();
      console.log(`initialise key with MyPDA at ${tx4}`);
    } catch {
      console.log(`This should also fail because keypair doesn't take seed and bump`);

    }
  });

  it("Writing to keypair account fails!", async () => {
    const newKeyPair = anchor.web3.Keypair.generate();
    const receiveWallet = anchor.web3.Keypair.generate();

    await airdropSol(newKeyPair.publicKey, 10);

    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeyPair.publicKey,
        toPubkey: receiveWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    )

    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [newKeyPair]
    );

    console.log('sent 1 lampport')

    await program.methods.initializeKeyPairAccount().accounts({
      storeKey: newKeyPair.publicKey
    })
    .signers([newKeyPair])
    .rpc()

    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeyPair.publicKey,
        toPubkey: receiveWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    )

    try {
      await anchor.web3.sendAndConfirmTransaction(
        anchor.getProvider().connection,
        transaction,
        [newKeyPair]
      );
      console.log('sent an other 1 lampport')
    } catch (err){
      console.log(err)
    }

  })

  it("Writing to keypair account fails!", async () => {
    console.log(`The program address is ${program.programId}`) 
    const newKeyPair = anchor.web3.Keypair.generate();

    await airdropSol(newKeyPair.publicKey, 10);
    const accountInfoBefore = await anchor.getProvider().connection.getAccountInfo(
      newKeyPair.publicKey);
    console.log(`initial keypair account owner is ${accountInfoBefore.owner}`);


    await program.methods.initializeKeyPairAccount().accounts({
      storeKey: newKeyPair.publicKey
    })
    .signers([newKeyPair])
    .rpc()

    const accountInfoAfter = await anchor.getProvider().connection.getAccountInfo(
      newKeyPair.publicKey);
    console.log(`initial keypair account owner is ${accountInfoAfter.owner}`);
  })

});
