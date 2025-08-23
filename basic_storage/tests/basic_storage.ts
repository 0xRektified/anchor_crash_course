import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicStorage } from "../target/types/basic_storage";

describe("basic_storage", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicStorage as Program<BasicStorage>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("The storage account address is", myStorage.toBase58());

      const tx = await program.methods.initialize()
        .accounts({
          myStorage: myStorage,
        })
        .rpc();
  });

  it("Is set!", async () => {
    const seeds = [];

    const [myStorage, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,
    program.programId
    );
    const tx = await program.methods.set(
      new anchor.BN(170)
    )
    .accounts({
          myStorage: myStorage,
    })
    .rpc();

    console.log(`tx is`, tx);

    await program.methods.addOne(new anchor.BN(170)).accounts({myStorage: myStorage}).rpc();
    await program.methods.printX().accounts({myStorage: myStorage}).rpc();

    let myStorageStruct = await program.account.myStorage.fetch(myStorage);
    console.log("The value of x is:",myStorageStruct.x.toString());
  });
});


/*
 solana account 8e4cN6Lg7yJr4cpsqLt9tUmp89rphSDpJtH6WTFi4iS5                                                  dev@Gabriels-MBP

Public Key: 8e4cN6Lg7yJr4cpsqLt9tUmp89rphSDpJtH6WTFi4iS5
Balance: 0.00105792 SOL
Owner: E2CwkZmjHPHXathgKu2FQjFVt3B1kU9JQmqUMGz7uSPR
Executable: false
Rent Epoch: 18446744073709551615
Length: 24 (0x18) bytes
0000:   1c f2 3b 85  43 19 31 28  aa 00 00 00  00 00 00 00   ..;.C.1(........
0010:   00 00 00 00  00 00 00 00


The first 8 bytes are the discriminator.
Our test stored the number 170 in the struct,
this has a hex representation of aa
*/