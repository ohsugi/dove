import * as anchor from "@project-serum/anchor";
import { Program, web3, BN } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveFund, updateDoveFund, pullDoveCampaign, createDoveCampaign, sleep, getBalance, equalDateTime, getNow, deleteDoveFund, updateDoveCampaign, deleteDoveCampaign } from "./util";
import assert from 'assert';

describe("test_delete_dove_campaign", () => {
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

    it("deleteDoveCampaign", async () => {
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS);

        let doveCampaign = await createDoveCampaign(
            "",
            "Test Porject 7",
            "China",
            "",
            "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
            "",
            program,
            admin,
        );
        const dove_campaign_created_date = getNow();
        let doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(doveCampaignAccount.adminPubkey.toString(), admin.publicKey.toString());
        assert.equal(doveCampaignAccount.evidenceLink, "");
        assert.equal(doveCampaignAccount.campaignName, "Test Porject 7");
        assert.equal(doveCampaignAccount.targetCountryCode, "CN");
        assert.equal(doveCampaignAccount.opponentCountryCode, "");
        assert.equal(doveCampaignAccount.description, "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
        assert.ok(equalDateTime(doveCampaignAccount.createdDate, dove_campaign_created_date));
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, dove_campaign_created_date));
        assert.ok(!doveCampaignAccount.isLocked);
        assert.ok(!doveCampaignAccount.isDeleted);
        assert.equal(doveCampaignAccount.videoLink, "");
        assert.equal(doveCampaignAccount.amountPooled, 0);
        assert.equal(doveCampaignAccount.amountTransferred, 0);
        assert.equal(doveCampaignAccount.decision, 0);

        await sleep();

        let dove_campaign_lamports = await getBalance(program, doveCampaign);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS);

        // Create DoveFund0
        const transferred_lamports_by_user0 = 1.2 * web3.LAMPORTS_PER_SOL;
        const doveFund0 = await createDoveFund(
            doveCampaign,
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
        assert.equal(doveFundAccount0.campaignPubkey.toString(), doveCampaign.toString());
        assert.equal(doveFundAccount0.userPubkey.toString(), user0.publicKey.toString());
        assert.equal(doveFundAccount0.amountPooled, transferred_lamports_by_user0);
        assert.equal(doveFundAccount0.amountTransferred, 0);
        assert.equal(Math.round(doveFundAccount0.decision * 100) / 100, 0.3);
        assert.ok(!doveFundAccount0.showsUser);
        assert.ok(!doveFundAccount0.showsPooledAmount);
        assert.ok(doveFundAccount0.showsTransferredAmount);
        assert.ok(equalDateTime(doveFundAccount0.createdDate, dove_fund0_created_date));
        assert.ok(equalDateTime(doveFundAccount0.updateDate, dove_fund0_created_date));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(doveCampaignAccount.amountPooled.toNumber(), transferred_lamports_by_user0);
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(Math.round(doveCampaignAccount.decision * 100) / 100, 0.3);
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, dove_fund0_created_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        let dove_fund0_lamports = await getBalance(program, doveFund0);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_lamports);

        let errorMessage = "";
        try {
            await pullDoveCampaign(
                doveCampaign,
                new BN(transferred_lamports_by_user0 - 0.5 * web3.LAMPORTS_PER_SOL),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /InconsistentAmountPooled/);

        errorMessage = "";
        try {
            await pullDoveCampaign(
                doveCampaign,
                new BN(transferred_lamports_by_user0),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /PullFundsIsNotAllowed/);

        // Delete doveCampaign
        errorMessage = "";
        try {
            await deleteDoveCampaign(
                doveCampaign,
                program,
                user0,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /InvalidUser/);

        errorMessage = "";
        try {
            await deleteDoveCampaign(
                doveCampaign,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.equal(errorMessage, "");

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.ok(doveCampaignAccount.isDeleted);

        errorMessage = "";
        try {
            await updateDoveCampaign(
                doveCampaign,
                "https://twitter.com/Ohsugi/status/1616505441705463816?s=20&t=vofTMniwI3ysTx9wyxy8dA",
                "Test Porject 2",
                "Taiwan, Province of China[a]",
                "China",
                "This is the updated dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
                "https://www.youtube.com/watch?v=zcVfBMse1Uw&ab_channel=DATALab",
                false,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /DoveCampaignIsAlreadyDeleted/);

        errorMessage = "";
        try {
            await deleteDoveCampaign(
                doveCampaign,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /DoveCampaignIsAlreadyDeleted/);

        const transferred_lamports_by_user1 = 1.2 * web3.LAMPORTS_PER_SOL;
        errorMessage = "";
        try {
            await createDoveFund(
                doveCampaign,
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
        assert.match(errorMessage, /DoveCampaignIsAlreadyDeleted/);

        errorMessage = "";
        try {
            const updated_lamports_by_user0 = 1.3 * web3.LAMPORTS_PER_SOL;
            await updateDoveFund(
                doveCampaign,
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
        assert.match(errorMessage, /DoveCampaignIsAlreadyDeleted/);

        errorMessage = "";
        try {
            await deleteDoveFund(
                doveCampaign,
                program,
                user0,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.equal(errorMessage, "");
    });
});
