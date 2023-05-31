import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Counter } from "../target/types/counter";

describe("counter", () => {
    const provider = anchor.AnchorProvider.local();

    anchor.setProvider(provider);

    const program = anchor.workspace.Counter as Program<Counter>;

    let baseAccount = anchor.web3.Keypair.generate();

    // -- new changes --
    it("initializes the counter", async () => {
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
    });
    it("increments the counter", async () => {
        await program.methods
            .increment()
            .accounts({ baseAccount: baseAccount.publicKey })
            .signers([])
            .rpc();

        const incrementedCounter = await program.account.baseAccount.fetch(
            baseAccount.publicKey
        );

        assert.strictEqual(incrementedCounter.count.toNumber(), 1);
    });

    // -- new changes --
    it("decrements the counter", async () => {
        await program.methods
            .decrement()
            .accounts({ baseAccount: baseAccount.publicKey })
            .signers([])
            .rpc();

        const decrementedCounter = await program.account.baseAccount.fetch(
            baseAccount.publicKey
        );

        assert.strictEqual(decrementedCounter.count.toNumber(), 0);
    });
});
