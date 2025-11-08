import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import * as splToken from "@solana/spl-token";

import {PublicKey} from "@solana/web3.js";
import { assert} from 'chai';
import { Day34 } from "../target/types/day34";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";


describe("day34", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day34 as Program<Day34>;

  const provider = anchor.AnchorProvider.env();
  const signerKp = provider.wallet.payer;
  const toKp = new web3.Keypair();


  it("Creates a new mint and associated token account using CPI", async () => {

    // Derive mint address offchain
    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from("my_mint"), signerKp.publicKey.toBuffer()],
      program.programId
    );


    //get associated ATA, false here indicate if the ATA is an off curve address
    const ata = splToken.getAssociatedTokenAddressSync(mint, signerKp.publicKey, false);

    const tx = await program.methods
      .createAndMintToken()
      .accounts({
        signer: signerKp.publicKey,
        newMint: mint,
        newAta: ata,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

      console.log("Transaction signature:", tx);
      console.log("Token (Mint Account) Address:", mint.toString());
      console.log("Associated Token Account:", ata.toString());

      const mintInfo = await splToken.getMint(provider.connection, mint);
      assert.equal(mintInfo.decimals, 9, "Mint decimals should be 9");
      assert.equal(mintInfo.mintAuthority?.toString(), signerKp.publicKey.toString(), "Mint authority should be the signer");
      assert.equal(mintInfo.freezeAuthority?.toString(), signerKp.publicKey.toString(), "Freeze authority should be the signer");
      assert.equal(mintInfo.supply.toString(), "100000000000", "Supply should be 100 tokens (with 9 decimals)");

      const tokenAccount = await splToken.getAccount(provider.connection, ata);
      assert.equal(tokenAccount.mint.toString(), mint.toString(), "Token account mint should match the mint PDA");
      assert.equal(tokenAccount.owner.toString(), signerKp.publicKey.toString(), "Token account owner should be the signer");
      assert.equal(tokenAccount.amount.toString(), "100000000000", "Token balance should be 100 tokens (with 9 decimals)");
      assert.equal(tokenAccount.delegate, null, "Token account should not have a delegate");
      
  });

  it("Transfer tokens using CPI", async() => {
    
    // Derive Pda from the mint
    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from("my_mint"), signerKp.publicKey.toBuffer()],
      program.programId
    );

    // Get the ATAs
    const fromAta= splToken.getAssociatedTokenAddressSync(mint, signerKp.publicKey, false);
    const toAta= splToken.getAssociatedTokenAddressSync(mint, toKp.publicKey, false);

    try {
      await splToken.createAssociatedTokenAccount(
        provider.connection,
        signerKp,
        mint,
        toKp.publicKey
      );
    } catch(error) {
      throw new Error(error)
    }

    const transferAmount = new anchor.BN(10_000_000_000);

    const tx = await program.methods
    .transferTokens(transferAmount)
    .accounts({
      from: signerKp.publicKey,
      fromAta: fromAta,
      toAta: toAta,
      tokenProgram: splToken.TOKEN_PROGRAM_ID
    })
    .rpc()

    console.log("Transfer Transaction signature:", tx);

    const toBalance = await provider.connection.getTokenAccountBalance(toAta);
    assert.equal(toBalance.value.amount, transferAmount.toString(), "Recipient balance should match transfer amount")

  })

  it("Reads token balance using CPI", async() => {
    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from("my_mint"), signerKp.publicKey.toBuffer()],
      program.programId
    );

    const ata = splToken.getAssociatedTokenAddressSync(mint, signerKp.publicKey, false);

    const tx = await program.methods.getBalance().accounts({
      tokenAccount: ata,
    }).rpc();

    console.log("Get Balance transaction Signature", tx);

    const balance = await provider.connection.getTokenAccountBalance(ata);
    assert.isTrue(balance.value.uiAmount > 0 , "Token balance should be greater than 0");
  })

  it("Disable authority", async() => {
    const newSigner = new web3.Keypair();
    let airdroptx = await anchor.getProvider().connection.requestAirdrop(newSigner.publicKey, 1e9);
    const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdroptx,
    })

    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from("my_mint"), newSigner.publicKey.toBuffer()],
      program.programId
    );

    const ata = splToken.getAssociatedTokenAddressSync(mint, newSigner.publicKey, false);


    const tx = await program.methods
      .createAndMintToken()
      .accounts({
        signer: newSigner.publicKey,
        newMint: mint,
        newAta: ata,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newSigner])
      .rpc();
    console.log("Mint and transfer", tx);

    const initialMint = await splToken.getAccount(provider.connection,ata); 

    console.log("Initial mint value", initialMint.amount);
  
    const tx2 = await program.methods.disableMintAuthority()
    .accounts({
      signer: newSigner.publicKey,
      mint: mint,
      tokenProgram: splToken.TOKEN_PROGRAM_ID,
    })
    .signers([newSigner])
    .rpc()
    console.log("disable authority", tx);

    try {
      await splToken.mintTo(
        provider.connection,
        signerKp,
        mint,
        ata,
        newSigner,
        100_000_000
      );
      assert.fail("Mint should have failed but succeeded");
    } catch(error){
      console.log("Mint correctly failed")
    }

  })
});
