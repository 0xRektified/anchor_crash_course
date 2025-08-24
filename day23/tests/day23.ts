import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day23 } from "../target/types/day23";

describe("day23", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.day23 as Program<Day23>;
  
  async function printAccountBalance(account, account2) {
    const balance = await anchor.getProvider().connection
      .getBalance(account);
      const balance2 = await anchor.getProvider().connection
      .getBalance(account2);
      console.log(`account: ${account} has ${balance/anchor.web3.LAMPORTS_PER_SOL} SOL`)
      console.log(`account: ${account2} has ${balance/anchor.web3.LAMPORTS_PER_SOL} SOL`)
  }


  it("Is initialized!", async () => {
    // Add your test here.
    const recipient = anchor.web3.Keypair.generate();
    const recipient2 = anchor.web3.Keypair.generate();

    await printAccountBalance(recipient.publicKey, recipient2.publicKey);
  
    const accountMeta = {pubkey: recipient.publicKey, isWritable: true, isSigner: false};
    const accountMeta2 = {pubkey: recipient2.publicKey, isWritable: true, isSigner: false};

    let amount = new anchor.BN(1*anchor.web3.LAMPORTS_PER_SOL);

    await program.methods.sendSol(amount)
    .remainingAccounts([
      accountMeta, accountMeta2
    ])
    .rpc();
    await printAccountBalance(recipient.publicKey, recipient2.publicKey);

  });
});
