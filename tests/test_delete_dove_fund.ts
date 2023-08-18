import * as anchor from "@project-serum/anchor";
import { Program, web3, BN } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveFund, updateDoveFund, createDoveCampaign, sleep, getBalance, deleteDoveFund, equalDateTime, getNow } from "./util";
import assert from 'assert';

describe("test_delete_dove_fund", () => {
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

    it("deleteDoveFund", async () => {
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS);

        const doveCampaign = await createDoveCampaign(
            "",
            "Test Porject 5",
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
        assert.equal(doveCampaignAccount.campaignName, "Test Porject 5");
        assert.equal(doveCampaignAccount.targetCountryCode, "JP");
        assert.equal(doveCampaignAccount.opponentCountryCode, "");
        assert.equal(doveCampaignAccount.description, "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
        assert.ok(equalDateTime(doveCampaignAccount.createdDate.toNumber(), dove_campaign_created_date));
        assert.ok(equalDateTime(doveCampaignAccount.updateDate.toNumber(), dove_campaign_created_date));
        assert.ok(!doveCampaignAccount.isLocked);
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
        assert.ok(equalDateTime(doveFundAccount0.createdDate.toNumber(), dove_fund0_created_date));
        assert.ok(equalDateTime(doveFundAccount0.updateDate.toNumber(), dove_fund0_created_date));

        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(doveCampaignAccount.amountPooled.toNumber(), transferred_lamports_by_user0);
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(Math.round(doveCampaignAccount.decision * 100) / 100, 0.2);
        assert.ok(equalDateTime(doveCampaignAccount.updateDate.toNumber(), dove_fund0_created_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        let dove_fund0_lamports = await getBalance(program, doveFund0);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_lamports);

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
        assert.ok(equalDateTime(doveFundAccount1.createdDate.toNumber(), dove_fund1_created_date));
        assert.ok(equalDateTime(doveFundAccount1.updateDate.toNumber(), dove_fund1_created_date));

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
        assert.ok(equalDateTime(doveCampaignAccount.updateDate.toNumber(), dove_fund1_created_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        assert.equal(await getBalance(program, doveFund0), dove_fund0_lamports);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS - dove_fund0_lamports);
        let dove_fund1_lamports = await getBalance(program, doveFund1);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);

        // Delete DoveFund0
        doveFund0 = await deleteDoveFund(
            doveCampaign,
            program,
            user0,
        );

        const dove_fund0_update_date = getNow();
        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(
            doveCampaignAccount.amountPooled.toNumber(),
            transferred_lamports_by_user1
        );
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(
            Math.round(doveCampaignAccount.decision * 100) / 100,
            Math.round((transferred_lamports_by_user1 * 0.3) / transferred_lamports_by_user1 * 100) / 100
        );
        assert.ok(equalDateTime(doveCampaignAccount.updateDate.toNumber(), dove_fund0_update_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS - dove_fund1_lamports);

        // Delete DoveFund1
        doveFund1 = await deleteDoveFund(
            doveCampaign,
            program,
            user1,
        );

        const dove_fund1_update_date = getNow();
        doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
        assert.equal(doveCampaignAccount.amountPooled.toNumber(), 0);
        assert.equal(doveCampaignAccount.amountTransferred.toNumber(), 0);
        assert.equal(Math.round(doveCampaignAccount.decision * 100) / 100, 0.5);
        assert.ok(equalDateTime(doveCampaignAccount.updateDate.toNumber(), dove_fund1_update_date));

        await sleep();

        assert.equal(await getBalance(program, doveCampaign), dove_campaign_lamports);
        assert.equal(await getBalance(program, admin.publicKey), DEFAULT_LAMPORTS - dove_campaign_lamports);
        assert.equal(await getBalance(program, user0.publicKey), DEFAULT_LAMPORTS);
        assert.equal(await getBalance(program, user1.publicKey), DEFAULT_LAMPORTS);
    });
});
