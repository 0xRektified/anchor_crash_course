import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day31 } from "../target/types/day31";

describe("day31", () => {
  async function airdropSol(publicKey, amount) {    
    let airdropTx = await anchor
        .getProvider()
        .connection.requestAirdrop(
            publicKey, 
            amount * anchor.web3.LAMPORTS_PER_SOL
        );  

    await confirmTransaction(airdropTx);  
}  

async function confirmTransaction(tx) {    
    const latestBlockHash = await anchor
        .getProvider()
        .connection.getLatestBlockhash();

    await anchor
        .getProvider()
        .connection.confirmTransaction({      
            blockhash: latestBlockHash.blockhash,      	
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,      
            signature: tx,    
    });  
}  

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day31 as Program<Day31>;
  const wallet = anchor.workspace.day31.provider.wallet;

  it("Is initialized!", async () => {
    const newKeyPair = anchor.web3.Keypair.generate();
    await airdropSol(newKeyPair.publicKey, 10);


    try {
      await program.methods.initialize().accounts({someAccount: newKeyPair.publicKey}).rpc();
    } catch (err){
      console.log(err)
      console.log("Fail because there is no init maccro check  https://docs.rs/anchor-lang/latest/anchor_lang/accounts/account/struct.Account.html");
    }
  });

  it("Load account iwth accountInfo", async()=>{
    const newKeyPair = anchor.web3.Keypair.generate();

      const tx = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: wallet.publicKey,
          newAccountPubkey: newKeyPair.publicKey,
          space: 16,
          lamports: await anchor.getProvider().connection.getMinimumBalanceForRentExemption(32),
          programId: program.programId,
        })
      );
      await anchor.web3.sendAndConfirmTransaction(
        anchor.getProvider().connection,
        tx,
        [wallet.payer, newKeyPair]
      );

      await program.methods.uncheckRead().accounts({
        uncheckRead: newKeyPair.publicKey
      }).rpc();
  })

  it("Load account iwth accountInfo", async()=>{
    await program.methods.hello().rpc();
  })
});
