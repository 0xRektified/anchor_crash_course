import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.sysvar as Program<Sysvar>;

  const StakeHistory_PublicKey = new anchor.web3.PublicKey(
    "SysvarStakeHistory1111111111111111111111111"
  )

  const LastRestartSlot_PublicKey = new anchor.web3.PublicKey(
    "SysvarLastRestartS1ot1111111111111111111111"
  )

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize(3)
    .accounts({
      stakeHistory: StakeHistory_PublicKey,
      recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
      instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      lastRestartSlot: LastRestartSlot_PublicKey,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Get day of the week!", async () => {
    // Add your test here.
    const tx = await program.methods
      .getDayOfWeek()
      .accounts({
        stakeHistory: StakeHistory_PublicKey,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        lastRestartSlot: LastRestartSlot_PublicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

});
