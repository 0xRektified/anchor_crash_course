import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DeployTutorial } from "../target/types/deploy_tutorial";

import fs from 'fs'
let idl = JSON.parse(fs.readFileSync('target/idl/deploy_tutorial.json', 'utf-8'))

// use
// `anchor test --skip-local-validator --skip-deploy`
// to avoid deploying each test
describe("deploy_tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = new Program(idl, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
