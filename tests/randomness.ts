import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Randomness } from "../target/types/randomness";
import {
  PublicKey,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";

describe("randomness", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Randomness as Program<Randomness>;

  it("Is initialized!", async () => {
    // Add your test here.

    let tx = await provider.send(
      new Transaction().add(
        //program.instruction.doNothing({})
        // program.instruction.doNothing({}),
        // program.instruction.doNothing({}),
        program.instruction.initialize({
          accounts: {
            instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            extra: provider.wallet.publicKey,
          },
        })
      )
    );
    // const tx = await program.rpc.initialize({
    //   accounts: {
    //     instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
    //   },
    //   preInstructions: [
    //     program.instruction.doNothing({}),
    //     program.instruction.doNothing({}),
    //   ],
    // });
    console.log("Your transaction signature", tx);
  });
});
