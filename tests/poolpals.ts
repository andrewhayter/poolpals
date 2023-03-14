import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Poolpals } from "../target/types/poolpals";

describe("poolpals", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Poolpals as Program<Poolpals>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
