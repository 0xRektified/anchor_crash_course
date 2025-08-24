import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ExampleMap } from "../target/types/example_map";

describe("example_map", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.exampleMap as Program<ExampleMap>;

  it("Initialize mapping storage", async () => {
    const key = new anchor.BN(42);



    const value = new anchor.BN(69);

    const seeds = [
      key.toArrayLike(Buffer, 'le', 8),
    ];

    // Trying to use the wrong PDA
    const category = new anchor.BN(1);
    const colateral = new anchor.BN(888);

    const dexSeeds = [
      category.toArrayLike(Buffer, 'le', 8),
      colateral.toArrayLike(Buffer, 'le', 8),
    ];


    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId,
    )[0];

    let valueDexAccount = anchor.web3.PublicKey.findProgramAddressSync(
      dexSeeds,
      program.programId,
    )[0];

    await program.methods.initialize(key)
    .accounts({
      val: valueAccount
    })
    .rpc();

    await program.methods.set(value)
    .accounts({
      val: valueAccount
    })
    .rpc();

    let result = await program.account.val.fetch(valueAccount);
    console.log(`the value ${result.value} wast stored in ${valueAccount.toBase58()}`);


    await program.methods.initializeDex(category, colateral)
    .accounts({
      dex: valueDexAccount
    })
    .rpc();

    // Set dex value
    await program.methods.setDex(value)
    .accounts({
      dex: valueDexAccount
    })
    .rpc();

    let result2 = await program.account.dex.fetch(valueDexAccount);
    console.log(`the value ${result2.liquidity} wast stored in ${valueDexAccount.toBase58()}`);
  });
});
