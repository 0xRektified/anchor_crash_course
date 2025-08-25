import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day29 } from "../target/types/day29";

describe("day29", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day29 as Program<Day29>;

  it("Is initialized!", async () => {
    const [store, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    )
    console.log(`program: ${program.programId.toBase58()}`);
    console.log(`storage account: ${store.toBase58()}`);
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    // Method 1: Calculate program data account PDA
    const [programDataAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [program.programId.toBuffer()],
      new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    );
    
    console.log(`Program data account: ${programDataAccount.toBase58()}`);
    
    const programDataInfo = await anchor.getProvider().connection.getAccountInfo(programDataAccount);
    console.log(`Program data length: ${programDataInfo?.data.length} bytes`);
    
    const programInfo = await anchor.getProvider().connection.getAccountInfo(program.programId);
    if (programInfo?.data.length === 36) {
      const dataAddress = new anchor.web3.PublicKey(programInfo.data.slice(4, 36));
      console.log(`Data address from program account: ${dataAddress.toBase58()}`);
    }
  });
});

/*

program: 6s26X8cev112oR96FZcWkkvtNEYBPPV9A5QeanShuPhR
storage account: FbnzNiqzZYzucRh8rXHRUzLiR8F2ACmDfkJPUZKj6hRz
Your transaction signature jYQXj5rPWRyraieXW9mJKcRbm8VAEGv2DcPLXzgoDYAyQQcD1Eqnj8Zkt8UBTdMGi4Hw1hCjsZ8keowipzEixYk

$solana account FbnzNiqzZYzucRh8rXHRUzLiR8F2ACmDfkJPUZKj6hRz                                       dev@Gabriels-MBP

Public Key: FbnzNiqzZYzucRh8rXHRUzLiR8F2ACmDfkJPUZKj6hRz
Balance: 0.00094656 SOL
Owner: 6s26X8cev112oR96FZcWkkvtNEYBPPV9A5QeanShuPhR <- onwer is program
Executable: false
Rent Epoch: 18446744073709551615
Length: 8 (0x8) bytes
0000:   82 30 f7 f4  b6 bf 1e 1a


~/git/solana/anchor/day29 (master*) » solana program show 6s26X8cev112oR96FZcWkkvtNEYBPPV9A5QeanShuPhR                                  dev@Gabriels-MBP

Program Id: 6s26X8cev112oR96FZcWkkvtNEYBPPV9A5QeanShuPhR
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: 4EGjEim8tCN4KKru3t5my4MEbA9xZyJaRsVmRpouzEjo <- data are located here
Authority: azAq71cZVjpiQvd7c4qAicrq1zQNM8Eb84xR8Y81w5o
Last Deployed In Slot: 28
Data Length: 207104 (0x32900) bytes
Balance: 1.44264792 SOL
*/
