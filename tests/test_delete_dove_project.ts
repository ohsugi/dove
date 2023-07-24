import * as anchor from "@project-serum/anchor";
import { Program, web3, BN } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveFund, updateDoveFund, pullDoveProject, createDoveProject, sleep, getBalance, equalDateTime, getNow, deleteDoveFund, updateDoveProject, deleteDoveProject } from "./util";
import assert from 'assert';

describe("test_delete_dove_project", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Dove as Program<Dove>;
    const DEFAULT_LAMPORTS: number = 4 * web3.LAMPORTS_PER_SOL;

    let admin: web3.Keypair;
    let user0: web3.Keypair;
    let user1: web3.Keypair;

    before(async () => {
        admin = await createUser(program, DEFAULT_LAMPORTS);
        user0 = await createUser(program, DEFAULT_LAMPORTS);
        user1 = await createUser(program, DEFAULT_LAMPORTS);
    })

    it("deleteDoveProject", async () => {
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS);

        let doveProject = await createDoveProject(
            "",
            "Test Porject 7",
            "China",
            "",
            "This is the test dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
            "",
            program,
            admin,
        );
        const dove_project_created_date = getNow();
        let doveProjectAccount = await program.account.doveProject.fetch(doveProject);
        assert.equal(doveProjectAccount.adminPubkey.toString(), admin.publicKey.toString());
        assert.equal(doveProjectAccount.evidenceLink, "");
        assert.equal(doveProjectAccount.projectName, "Test Porject 7");
        assert.equal(doveProjectAccount.targetCountryCode, "CN");
        assert.equal(doveProjectAccount.opponentCountryCode, "");
        assert.equal(doveProjectAccount.description, "This is the test dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
        assert.ok(equalDateTime(doveProjectAccount.createdDate, dove_project_created_date));
        assert.ok(equalDateTime(doveProjectAccount.updateDate, dove_project_created_date));
        assert.ok(!doveProjectAccount.isLocked);
        assert.ok(!doveProjectAccount.isDeleted);
        assert.equal(doveProjectAccount.videoLink, "");
        assert.equal(doveProjectAccount.amountPooled, 0);
        assert.equal(doveProjectAccount.amountTransferred, 0);
        assert.equal(doveProjectAccount.decision, 0);

        await sleep(1000);

        let dove_project_lamports = await getBalance(program, doveProject);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_project_lamports);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS);

        // Create DoveFund0
        const transferred_lamports_by_user0 = 1.2 * web3.LAMPORTS_PER_SOL;
        const doveFund0 = await createDoveFund(
            doveProject,
            new BN(transferred_lamports_by_user0),
            0.3,
            false,
            false,
            true,
            program,
            user0,
        );
        const dove_fund0_created_date = getNow();
        let doveFundAccount0 = await program.account.doveFund.fetch(doveFund0);
        assert.equal(doveFundAccount0.projectPubkey.toString(), doveProject.toString());
        assert.equal(doveFundAccount0.userPubkey.toString(), user0.publicKey.toString());
        assert.equal(doveFundAccount0.amountPooled, transferred_lamports_by_user0);
        assert.equal(doveFundAccount0.amountTransferred, 0);
        assert.equal(Math.round(doveFundAccount0.decision * 100) / 100, 0.3);
        assert.ok(!doveFundAccount0.showsUser);
        assert.ok(!doveFundAccount0.showsPooledAmount);
        assert.ok(doveFundAccount0.showsTransferredAmount);
        assert.ok(equalDateTime(doveFundAccount0.createdDate, dove_fund0_created_date));
        assert.ok(equalDateTime(doveFundAccount0.updateDate, dove_fund0_created_date));

        doveProjectAccount = await program.account.doveProject.fetch(doveProject);
        assert.equal(doveProjectAccount.amountPooled.toNumber(), transferred_lamports_by_user0);
        assert.equal(doveProjectAccount.amountTransferred.toNumber(), 0);
        assert.equal(Math.round(doveProjectAccount.decision * 100) / 100, 0.3);
        assert.ok(equalDateTime(doveProjectAccount.updateDate, dove_fund0_created_date));

        await sleep(1000);

        assert.equal(await getBalance(program, doveProject), dove_project_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_project_lamports);
        let dove_fund0_lamports = await getBalance(program, doveFund0);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_lamports);

        let errorMessage = "";
        try {
            await pullDoveProject(
                doveProject,
                new BN(transferred_lamports_by_user0 - 0.5 * web3.LAMPORTS_PER_SOL),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("InconsistentAmountPooled"));

        errorMessage = "";
        try {
            await pullDoveProject(
                doveProject,
                new BN(transferred_lamports_by_user0),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("PullDoveProjectIsNotAllowed"));

        // Delete doveProject
        errorMessage = "";
        try {
            await deleteDoveProject(
                doveProject,
                program,
                user0,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("InvalidUserToDeleteDoveProject"));

        errorMessage = "";
        try {
            await deleteDoveProject(
                doveProject,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.equal(errorMessage, "");

        doveProjectAccount = await program.account.doveProject.fetch(doveProject);
        assert.ok(doveProjectAccount.isDeleted);

        errorMessage = "";
        try {
            await updateDoveProject(
                doveProject,
                "https://twitter.com/Ohsugi/status/1616505441705463816?s=20&t=vofTMniwI3ysTx9wyxy8dA",
                "Test Porject 2",
                "Taiwan, Province of China[a]",
                "China",
                "This is the updated dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
                "https://www.youtube.com/watch?v=zcVfBMse1Uw&ab_channel=DATALab",
                false,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("DoveProjectIsAlreadyDeleted"));

        errorMessage = "";
        try {
            await deleteDoveProject(
                doveProject,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("DoveProjectIsAlreadyDeleted"));

        const transferred_lamports_by_user1 = 1.2 * web3.LAMPORTS_PER_SOL;
        errorMessage = "";
        try {
            await createDoveFund(
                doveProject,
                new BN(transferred_lamports_by_user1),
                0.3,
                false,
                false,
                true,
                program,
                user1,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("DoveProjectIsAlreadyDeleted"));

        errorMessage = "";
        try {
            const updated_lamports_by_user0 = 1.3 * web3.LAMPORTS_PER_SOL;
            await updateDoveFund(
                doveProject,
                new BN(updated_lamports_by_user0),
                0.4,
                false,
                false,
                true,
                program,
                user0,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.ok(errorMessage.includes("DoveProjectIsAlreadyDeleted"));

        errorMessage = "";
        try {
            await deleteDoveFund(
                doveProject,
                program,
                user0,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.equal(errorMessage, "");
    });
});
