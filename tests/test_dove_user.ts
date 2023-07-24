import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveUser, updateDoveUser, deleteDoveUser, getBalance, sleep, equalDateTime, getNow } from "./util";

import assert from 'assert';

describe("test_dove_user", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Dove as Program<Dove>;
    const DEFAULT_LAMPORTS: number = 4 * web3.LAMPORTS_PER_SOL;

    let user0: web3.Keypair;
    let user1: web3.Keypair;
    let user2: web3.Keypair;

    before(async () => {
        user0 = await createUser(program, DEFAULT_LAMPORTS);
        user1 = await createUser(program, DEFAULT_LAMPORTS);
        user2 = await createUser(program, DEFAULT_LAMPORTS);
    })

    it("createDoveUser", async () => {
        const doveUser = await createDoveUser(
            "User 0",
            "https://twitter.com/Ohsugi",
            "https://twitter.com/Ohsugi/status/1644117229875724306?s=20",
            true,
            program,
            user0,
        );
        const dove_user_created_date = getNow();
        const doveUserAccount = await program.account.doveUser.fetch(doveUser);
        assert.equal(doveUserAccount.userPubkey.toString(), user0.publicKey.toString());
        assert.equal(doveUserAccount.userName, "User 0");
        assert.equal(doveUserAccount.socialMediaLink, "https://twitter.com/Ohsugi");
        assert.equal(doveUserAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1644117229875724306?s=20");
        assert.ok(doveUserAccount.isShown);
        assert.ok(equalDateTime(doveUserAccount.createdDate, dove_user_created_date));
        assert.ok(equalDateTime(doveUserAccount.updateDate, dove_user_created_date));
    });

    it("updateDoveUser", async () => {
        let doveUser = await createDoveUser(
            "User 1",
            "https://twitter.com/Ohsugi",
            "https://twitter.com/Ohsugi/status/1644117229875724306?s=20",
            true,
            program,
            user1,
        );
        const dove_user_created_date = getNow();
        let doveUserAccount = await program.account.doveUser.fetch(doveUser);
        assert.equal(doveUserAccount.userPubkey.toString(), user1.publicKey.toString());
        assert.equal(doveUserAccount.userName, "User 1");
        assert.equal(doveUserAccount.socialMediaLink, "https://twitter.com/Ohsugi");
        assert.equal(doveUserAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1644117229875724306?s=20");
        assert.ok(doveUserAccount.isShown);
        assert.ok(equalDateTime(doveUserAccount.createdDate, dove_user_created_date));
        assert.ok(equalDateTime(doveUserAccount.updateDate, dove_user_created_date));

        doveUser = await updateDoveUser(
            "User 1 updated",
            "https://twitter.com/Ohsugi_updated",
            "https://twitter.com/Ohsugi/status/1644117229875724306",
            false,
            program,
            user1,
        );

        const dove_user_updated_date = getNow();
        doveUserAccount = await program.account.doveUser.fetch(doveUser);
        assert.equal(doveUserAccount.userPubkey.toString(), user1.publicKey.toString());
        assert.equal(doveUserAccount.userName, "User 1 updated");
        assert.equal(doveUserAccount.socialMediaLink, "https://twitter.com/Ohsugi_updated");
        assert.equal(doveUserAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1644117229875724306");
        assert.ok(!doveUserAccount.isShown);
        assert.ok(equalDateTime(doveUserAccount.createdDate, dove_user_created_date));
        assert.ok(equalDateTime(doveUserAccount.updateDate, dove_user_updated_date));
    });

    it("deleteDoveUser", async () => {
        let doveUser = await createDoveUser(
            "User 2",
            "https://twitter.com/Ohsugi",
            "https://twitter.com/Ohsugi/status/1644117229875724306?s=20",
            false,
            program,
            user2,
        );
        const dove_user_created_date = getNow();
        let doveUserAccount = await program.account.doveUser.fetch(doveUser);
        assert.equal(doveUserAccount.userPubkey.toString(), user2.publicKey.toString());
        assert.equal(doveUserAccount.userName, "User 2");
        assert.equal(doveUserAccount.socialMediaLink, "https://twitter.com/Ohsugi");
        assert.equal(doveUserAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1644117229875724306?s=20");
        assert.ok(!doveUserAccount.isShown);
        assert.ok(equalDateTime(doveUserAccount.createdDate, dove_user_created_date));
        assert.ok(equalDateTime(doveUserAccount.updateDate, dove_user_created_date));

        let dove_user_lamports = await getBalance(program, doveUser);
        assert.equal(await getBalance(program, user2.publicKey), DEFAULT_LAMPORTS - dove_user_lamports);

        doveUser = await deleteDoveUser(
            program,
            user2,
        );

        await sleep();
        assert.equal(await getBalance(program, user2.publicKey), DEFAULT_LAMPORTS);
    });
});
