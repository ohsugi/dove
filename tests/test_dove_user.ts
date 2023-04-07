import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveUser } from "./util";

import assert from 'assert';

describe("test_dove_user", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Dove as Program<Dove>;
    const DEFAULT_LAMPORTS: number = 4 * web3.LAMPORTS_PER_SOL;
    const ACCEPTABLE_DATE_ERROR = 10000000;

    let admin: web3.Keypair;

    before(async () => {
        admin = await createUser(program, DEFAULT_LAMPORTS);
    })

    it("createDoveUser", async () => {
        const doveUser = await createDoveUser(
            "User 1",
            "https://twitter.com/Ohsugi",
            "https://twitter.com/Ohsugi/status/1644117229875724306?s=20",
            true,
            program,
            admin,
        );
        const dove_user_created_date = Date.now();
        const doveUserAccount = await program.account.doveUser.fetch(doveUser);
        assert.equal(doveUserAccount.userName, "User 1");
        assert.equal(doveUserAccount.socialMediaLink, "https://twitter.com/Ohsugi");
        assert.equal(doveUserAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1644117229875724306?s=20");
        assert.equal(doveUserAccount.isShown, true);
        assert.equal(doveUserAccount.amountPooled, 0);
        assert.equal(doveUserAccount.amountTransferred, 0);
        assert.ok(doveUserAccount.createdDate.toNumber() - dove_user_created_date < ACCEPTABLE_DATE_ERROR);
        assert.ok(doveUserAccount.updateDate.toNumber() - dove_user_created_date < ACCEPTABLE_DATE_ERROR);
    });
});
