import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { use } from "chai";

describe("anchor-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env() 
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

  // Initializing escrow dependencies

  const escrowState =  anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("state"), provider.publicKey.toBytes()], program.programId)[0];

  const escrow = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("state"), escrowState.toBytes()], program.programId)[0];

  // Initializing tests

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


  it("Make a deposit 4 sol", async () => {
    // Add your test here.
    const tx = await program.methods
    .make(
      new ancho.BN(4 * anchor.web3.LAMPORTS_PER_SOL)
    )
    .accountsPartial(
      {
        marker: provider.wallet.publicKey,
        escrowState,
        escrow,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    )
    .rpc();
    console.log("Your transaction signature", tx);
    console.log("Escrow deposit: ", (await provider.connection.getAccountInfo(escrow)));
  });
});
