import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day32 } from "../target/types/day32";
import { DataReader } from "../target/types/data_reader";

describe("day32", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const day32Program = anchor.workspace.day32 as Program<Day32>;
  const dataReaderProgram = anchor.workspace.dataReader as Program<DataReader>;

  it("Initialize day32 and read data with data_reader", async () => {
    const seeds = [];
    const [storage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      day32Program.programId
    );

    await day32Program.methods
      .initialize()
      .accounts({ storage: storage })
      .rpc();

    let storageStruct = await day32Program.account.storage.fetch(
      storage
    );

    console.log("Day32 - The value of x is:", storageStruct.x.toString());
    console.log("Storage account address:", storage.toBase58());

    await dataReaderProgram.methods
      .readOtherData()
      .accounts({ otherData: storage })
      .rpc();

    console.log("Successfully read data from day32 using data_reader!");
  });
});
