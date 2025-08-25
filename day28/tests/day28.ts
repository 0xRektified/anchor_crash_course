import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day28 } from "../target/types/day28";

describe("day28", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day28 as Program<Day28>;

  it("Is initialized!", async () => {
    const wallet = program.provider.wallet.payer;

    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId,
    );

    const initTx = await program.methods.initialize()
    .accounts({
      pda: pda
    })
    .transaction();

    const setTx = await program.methods.set(5)
    .accounts({
      pda: pda
    })
    .transaction();

    let transaction = new anchor.web3.Transaction();
    transaction.add(initTx);
    transaction.add(setTx);

    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [wallet]
    );

    const pdaAcc = await program.account.myPda.fetch(pda);

    console.log("Your transaction signature", pdaAcc.pda);
  });

  it("Is atomic tx", async () => {
    const wallet = anchor.web3.Keypair.generate();

    const signature = await anchor.getProvider().connection.requestAirdrop(
      wallet.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await anchor.getProvider().connection.confirmTransaction(signature)

    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );

    let tx = new anchor.web3.Transaction();

    let accountInfo = await anchor.getProvider().connection.getAccountInfo(pda);
    console.log(`accountInfo is null: ${accountInfo ? false: true} lamports: ${accountInfo.lamports}`)

    tx.add(
      await program.methods.set(5).accounts({
        pda: pda,
        signer: wallet.publicKey
      })
      .signers([wallet])
      .transaction()
    )

    tx.add(
      await program.methods.alwaysFail().accounts({
        pda: pda,
        signer: wallet.publicKey
      })
      .signers([wallet])
      .transaction()
    )
    
    try {
      await anchor.web3.sendAndConfirmTransaction(
        anchor.getProvider().connection,
        tx,
        [wallet])
    } catch (err){
      console.log(err)
    }

  })
});
