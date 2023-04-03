import * as anchor from "@project-serum/anchor";
import { expect } from "chai";
import { Program } from "@project-serum/anchor";
import { Poolpals } from "../target/types/poolpals";
const { SystemProgram } = anchor.web3;

describe("poolpals", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Poolpals as Program<Poolpals>;

  it("Is initialized!", async () => {
    const lottoAccount = await anchor.web3.Keypair.generate();

    let [globalStakePDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global_stake")],
      program.programId
    );

    const tx = await program.methods
      .initialize()
      .accounts({
        lottery: program.provider.publicKey,
        globalStake: globalStakePDA,
        initializer: lottoAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([lottoAccount])
      .rpc();

    console.log(tx);

    // const account = await program.account.lottery.fetch(lottoAccount.publicKey);

    // // expect(account.isInitalized == true);
    // expect(account.isInitalized).to.be.true;
  });
});
