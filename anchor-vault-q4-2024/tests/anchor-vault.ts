import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;


  // Initializing vault dependencies

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("state"), anchor.AnchorProvider.env().publicKey.toBytes()], program.programId)[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), vaultState.toBytes()], program.programId)[0];

  // Initialize function test

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accountsPartial({
      user: anchor.AnchorProvider.env().wallet.publicKey,
      vaultState,
      vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature: ", tx);
    console.log("Your vault state: ", (await anchor.AnchorProvider.env().connection.getAccountInfo(vault)));


  });

  // Deposit function test

  it("Deposit 4 sol", async () => {
    const tx = await program.methods
    .deposit(new anchor.BN(4 * anchor.web3.LAMPORTS_PER_SOL))
    .accountsPartial({
      user: await anchor.AnchorProvider.env().wallet.publicKey,
      vaultState,
      vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    console.log("Your transaction signature: ", tx);
    console.log("Your vault deposit: ", (await anchor.AnchorProvider.env().connection.getBalance(vault)).toString());
  });

  

  // Withdraw function test
  it("Withdraw 2 sol", async () => {
    const tx = await program.methods
    .withdraw(new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL))
    .accountsPartial({
      user: anchor.AnchorProvider.env().wallet.publicKey,
      vaultState,
      vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature: ", tx);
    console.log("Your vault deposit: ", (await anchor.AnchorProvider.env().connection.getBalance(vault)).toString());
  });

  // Close function test
  it("Close vault", async () => {
    const tx = await program.methods
    .close()
    .accountsPartial({
      vaultState,
      vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature: ", tx);
    console.log("Your vault state: ", (await anchor.AnchorProvider.env().connection.getAccountInfo(vault)));
  });

  
});
