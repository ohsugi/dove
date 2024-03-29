import * as anchor from "@project-serum/anchor";
import { Program, web3, BN } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveFund, updateDoveFund, pullDoveCampaign, createDoveCampaign, sleep, getBalance, equalDateTime, getNow, pullDoveFund } from "./util";
import assert from 'assert';

describe("test_pull_dove_fund", () => {
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

    it("pullDoveFund", async () => {
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS);

        const doveCampaign = await createDoveCampaign(
            "",
            "Test Porject 6",
            "Japan",
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
        assert.equal(doveCampaignAccount.campaignName, "Test Porject 6");
        assert.equal(doveCampaignAccount.targetCountryCode, "JP");
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
        const transferred_lamports_by_user0 = 1.1 * web3.LAMPORTS_PER_SOL;
        let doveFund0 = await createDoveFund(
            doveCampaign,
            new BN(transferred_lamports_by_user0),
            0.2,
            true,
            true,
            false,
            program,
            user0,
        );
        const dove_fund0_created_date = getNow();
        let doveFundAccount0 = await program.account.doveFund.fetch(doveFund0);
        assert.equal(doveFundAccount0.campaignPubkey.toString(), doveCampaign.toString());
        assert.equal(doveFundAccount0.userPubkey.toString(), user0.publicKey.toString());
        assert.equal(doveFundAccount0.amountPooled, transferred_lamports_by_user0);
        assert.equal(doveFundAccount0.amountTransferred, 0);
        assert.equal(Math.round(doveFundAccount0.decision * 100) / 100, 0.2);
        assert.ok(doveFundAccount0.showsUser);
        assert.ok(doveFundAccount0.showsPooledAmount);
        assert.ok(!doveFundAccount0.showsTransferredAmount);
        assert.ok(equalDateTime(doveFundAccount0.createdDate, dove_fund0_created_date));
        assert.ok(equalDateTime(doveFundAccount0.updateDate, dove_fund0_created_date));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(doveCampaignAccount.amountPooled.toNumber(), transferred_lamports_by_user0);
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(Math.round(doveCampaignAccount.decision * 100) / 100, 0.2);
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

        errorMessage = "";
        try {
            await pullDoveFund(
                doveFund0,
                doveCampaign,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /PullFundsIsNotAllowed/);

        // Create DoveFund1
        const transferred_lamports_by_user1 = 1.2 * web3.LAMPORTS_PER_SOL;
        let doveFund1 = await createDoveFund(
            doveCampaign,
            new BN(transferred_lamports_by_user1),
            0.3,
            false,
            false,
            true,
            program,
            user1,
        );
        const dove_fund1_created_date = getNow();
        let doveFundAccount1 = await program.account.doveFund.fetch(doveFund1);
        assert.equal(doveFundAccount1.campaignPubkey.toString(), doveCampaign.toString());
        assert.equal(doveFundAccount1.userPubkey.toString(), user1.publicKey.toString());
        assert.equal(doveFundAccount1.amountPooled, transferred_lamports_by_user1);
        assert.equal(doveFundAccount1.amountTransferred, 0);
        assert.equal(Math.round(doveFundAccount1.decision * 100) / 100, 0.3);
        assert.ok(!doveFundAccount1.showsUser);
        assert.ok(!doveFundAccount1.showsPooledAmount);
        assert.ok(doveFundAccount1.showsTransferredAmount);
        assert.ok(equalDateTime(doveFundAccount1.createdDate, dove_fund1_created_date));
        assert.ok(equalDateTime(doveFundAccount1.updateDate, dove_fund1_created_date));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(
            doveCampaignAccount.amountPooled.toNumber(),
            transferred_lamports_by_user0 + transferred_lamports_by_user1
        );
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(
            Math.round(doveCampaignAccount.decision * 100) / 100,
            Math.round((transferred_lamports_by_user0 * 0.2 + transferred_lamports_by_user1 * 0.3) / (transferred_lamports_by_user0 + transferred_lamports_by_user1) * 100) / 100
        );
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, dove_fund1_created_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        assert.equal(await getBalance(program, doveFund0), dove_fund0_lamports);
        let dove_fund1_lamports = await getBalance(program, doveFund1);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);

        errorMessage = "";
        try {
            await pullDoveCampaign(
                doveCampaign,
                new BN(transferred_lamports_by_user0 + transferred_lamports_by_user1 - 0.5 * web3.LAMPORTS_PER_SOL),
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
                new BN(transferred_lamports_by_user0 + transferred_lamports_by_user1),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /PullFundsIsNotAllowed/);

        errorMessage = "";
        try {
            await pullDoveFund(
                doveFund1,
                doveCampaign,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /PullFundsIsNotAllowed/);

        // Update DoveFund0
        const updated_lamports_by_user0 = 1.3 * web3.LAMPORTS_PER_SOL;
        doveFund0 = await updateDoveFund(
            doveCampaign,
            new BN(updated_lamports_by_user0),
            1.0,
            false,
            false,
            true,
            program,
            user0,
        );

        const dove_fund0_update_date = getNow();
        doveFundAccount0 = await program.account.doveFund.fetch(doveFund0);
        assert.equal(doveFundAccount0.campaignPubkey.toString(), doveCampaign.toString());
        assert.equal(doveFundAccount0.userPubkey.toString(), user0.publicKey.toString());
        assert.equal(doveFundAccount0.amountPooled.toNumber(), updated_lamports_by_user0);
        assert.equal(doveFundAccount0.amountTransferred, 0);
        assert.equal(Math.round(doveFundAccount0.decision * 100) / 100, 1.0);
        assert.ok(!doveFundAccount0.showsUser);
        assert.ok(!doveFundAccount0.showsPooledAmount);
        assert.ok(doveFundAccount0.showsTransferredAmount);
        assert.ok(equalDateTime(doveFundAccount0.createdDate, dove_fund0_created_date));
        assert.ok(equalDateTime(doveFundAccount0.updateDate, dove_fund0_update_date));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(
            doveCampaignAccount.amountPooled.toNumber(),
            updated_lamports_by_user0 + transferred_lamports_by_user1
        );
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(
            Math.round(doveCampaignAccount.decision * 100) / 100,
            Math.round((updated_lamports_by_user0 * 1.0 + transferred_lamports_by_user1 * 0.3) / (updated_lamports_by_user0 + transferred_lamports_by_user1) * 100) / 100
        );
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, dove_fund0_update_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        let dove_fund0_updated_lamports = await getBalance(program, doveFund0);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_updated_lamports);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);

        errorMessage = "";
        try {
            await pullDoveCampaign(
                doveCampaign,
                new BN(updated_lamports_by_user0 + transferred_lamports_by_user1 - 0.5 * web3.LAMPORTS_PER_SOL),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /InconsistentAmountPooled/);

        errorMessage = "";
        try {
            await pullDoveFund(
                doveFund0,
                doveCampaign,
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.match(errorMessage, /DoveCampaignIsNotLocked/);

        const pull_dove_campaign_date = getNow();
        errorMessage = "";
        try {
            await pullDoveCampaign(
                doveCampaign,
                new BN(updated_lamports_by_user0 + transferred_lamports_by_user1),
                program,
                admin,
            );
        } catch (e) {
            errorMessage = e.message;
        }
        assert.equal(errorMessage, "");

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(
            doveCampaignAccount.amountPooled.toNumber(),
            updated_lamports_by_user0 + transferred_lamports_by_user1
        );
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(
            Math.round(doveCampaignAccount.decision * 100) / 100,
            Math.round((updated_lamports_by_user0 * 1.0 + transferred_lamports_by_user1 * 0.3) / (updated_lamports_by_user0 + transferred_lamports_by_user1) * 100) / 100
        );
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, pull_dove_campaign_date));
        assert.ok(doveCampaignAccount.isLocked);

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        let dove_fund0_lamports_before_pull = await getBalance(program, doveFund0);
        assert.equal(await getBalance(program, doveFund0), dove_fund0_lamports_before_pull);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_updated_lamports);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);

        await pullDoveFund(
            doveFund0,
            doveCampaign,
            program,
            admin,
        );

        doveFundAccount0 = await program.account.doveFund.fetch(doveFund0);
        assert.equal(doveFundAccount0.campaignPubkey.toString(), doveCampaign.toString());
        assert.equal(doveFundAccount0.userPubkey.toString(), user0.publicKey.toString());
        assert.equal(doveFundAccount0.amountPooled.toNumber(), 0.0);
        assert.equal(doveFundAccount0.amountTransferred, updated_lamports_by_user0);
        assert.equal(Math.round(doveFundAccount0.decision * 100) / 100, 1.0);
        assert.ok(!doveFundAccount0.showsUser);
        assert.ok(!doveFundAccount0.showsPooledAmount);
        assert.ok(doveFundAccount0.showsTransferredAmount);
        assert.ok(equalDateTime(doveFundAccount0.createdDate, dove_fund0_created_date));
        assert.ok(equalDateTime(doveFundAccount0.updateDate, doveCampaignAccount.updateDate));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(
            doveCampaignAccount.amountPooled.toNumber(),
            transferred_lamports_by_user1
        );
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), updated_lamports_by_user0);
        assert.equal(
            Math.round(doveCampaignAccount.decision * 100) / 100,
            Math.round((updated_lamports_by_user0 * 1.0 + transferred_lamports_by_user1 * 0.3) / (updated_lamports_by_user0 + transferred_lamports_by_user1) * 100) / 100
        );
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, doveFundAccount0.updateDate));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports + updated_lamports_by_user0);
        assert.equal(await getBalance(program, doveFund0), dove_fund0_lamports_before_pull - updated_lamports_by_user0);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_updated_lamports);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);

        let dove_fund1_lamports_before_pull = await getBalance(program, doveFund1);

        await pullDoveFund(
            doveFund1,
            doveCampaign,
            program,
            admin,
        );

        doveFundAccount1 = await program.account.doveFund.fetch(doveFund1);
        assert.equal(doveFundAccount1.campaignPubkey.toString(), doveCampaign.toString());
        assert.equal(doveFundAccount1.userPubkey.toString(), user1.publicKey.toString());
        assert.equal(doveFundAccount1.amountPooled.toNumber(), 0.0);
        assert.equal(doveFundAccount1.amountTransferred, transferred_lamports_by_user1);
        assert.equal(Math.round(doveFundAccount1.decision * 100) / 100, 0.3);
        assert.ok(!doveFundAccount1.showsUser);
        assert.ok(!doveFundAccount1.showsPooledAmount);
        assert.ok(equalDateTime(doveFundAccount1.createdDate, dove_fund1_created_date));
        assert.ok(equalDateTime(doveFundAccount1.updateDate, doveCampaignAccount.updateDate));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(doveCampaignAccount.amountPooled.toNumber(), 0);
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), updated_lamports_by_user0 + transferred_lamports_by_user1);
        assert.equal(
            Math.round(doveCampaignAccount.decision * 100) / 100,
            Math.round((updated_lamports_by_user0 * 1.0 + transferred_lamports_by_user1 * 0.3) / (updated_lamports_by_user0 + transferred_lamports_by_user1) * 100) / 100
        );
        assert.ok(equalDateTime(doveCampaignAccount.updateDate, doveFundAccount1.updateDate));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports + updated_lamports_by_user0 + transferred_lamports_by_user1);
        assert.equal(await getBalance(program, doveFund0), dove_fund0_lamports_before_pull - updated_lamports_by_user0);
        assert.equal(await getBalance(program, doveFund1), dove_fund1_lamports_before_pull - transferred_lamports_by_user1);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_updated_lamports);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);
    });
});
