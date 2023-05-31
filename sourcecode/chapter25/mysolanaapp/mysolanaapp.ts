import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Mysolanaapp } from "../target/types/mysolanaapp";

describe("mysolanaapp", () => {
	const provider = anchor.AnchorProvider.local();
        anchor.setProvider(provider);
        const program = anchor.workspace.Mysolanaapp as Program<Mysolanaapp>;
        let baseAccount = anchor.web3.Keypair.generate();

/*	it("initializes the counter", async () => {
	   await program.methods
	            .initialize()
	            .accounts({
	                   baseAccount: baseAccount.publicKey,
	                   user: provider.wallet.publicKey,
	                   systemProgram: anchor.web3.SystemProgram.programId,
	             })
	            .signers([baseAccount])
	            .rpc();

            const createdCounter = await program.account.baseAccount.fetch(
                      baseAccount.publicKey
	    );
            assert.strictEqual(createdCounter.count.toNumber(), 0);
	});*/
});
