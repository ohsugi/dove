import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveCampaign, updateDoveCampaign, sleep, equalDateTime, getNow } from "./util";

import assert from 'assert';

describe("test_dove_campaign", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Dove as Program<Dove>;
  const DEFAULT_LAMPORTS: number = 4 * web3.LAMPORTS_PER_SOL;

  let admin: web3.Keypair;

  before(async () => {
    admin = await createUser(program, DEFAULT_LAMPORTS);
  })

  // This test case was not intended for the actual potential conflict of the country code, but only test.
  it("createDoveCampaign", async () => {
    const doveCampaign = await createDoveCampaign(
      "",
      "Test Porject 0",
      "United States of America",
      "Cayman Islands",
      "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
      "",
      program,
      admin,
    );
    const dove_campaign_created_date = getNow();
    const doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
    assert.equal(doveCampaignAccount.adminPubkey.toString(), admin.publicKey.toString());
    assert.equal(doveCampaignAccount.evidenceLink, "");
    assert.equal(doveCampaignAccount.campaignName, "Test Porject 0");
    assert.equal(doveCampaignAccount.targetCountryCode, "US");
    assert.equal(doveCampaignAccount.opponentCountryCode, "KY");
    assert.equal(doveCampaignAccount.description, "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
    assert.equal(doveCampaignAccount.createdDate.toNumber(), doveCampaignAccount.updateDate.toNumber());
    assert.ok(!doveCampaignAccount.isLocked);
    assert.equal(doveCampaignAccount.videoLink, "");
    assert.equal(doveCampaignAccount.amountPooled, 0);
    assert.equal(doveCampaignAccount.amountTransferred, 0);
    assert.equal(doveCampaignAccount.decision, 0);
    assert.ok(equalDateTime(doveCampaignAccount.createdDate.toNumber(), dove_campaign_created_date));
    assert.ok(equalDateTime(doveCampaignAccount.lastDateTransferred.toNumber(), dove_campaign_created_date));
  });

  it("updateDoveCampaign", async () => {
    const doveCampaign = await createDoveCampaign(
      "https://twitter.com/Ohsugi/status/1615827817627017217?s=20&t=gFmtF8G4VrnDrzB0jhCsRA",
      "Test Porject 1",
      "Japan",
      "",
      "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
      "https://youtu.be/zcVfBMse1Uw",
      program,
      admin,
    );
    const dove_campaign_created_date = getNow();
    const doveCampaignAccount = await program.account.doveCampaign.fetch(doveCampaign);
    assert.equal(doveCampaignAccount.adminPubkey.toString(), admin.publicKey.toString());
    assert.equal(doveCampaignAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1615827817627017217?s=20&t=gFmtF8G4VrnDrzB0jhCsRA");
    assert.equal(doveCampaignAccount.campaignName, "Test Porject 1");
    assert.equal(doveCampaignAccount.targetCountryCode, "JP");
    assert.equal(doveCampaignAccount.opponentCountryCode, "");
    assert.equal(doveCampaignAccount.description, "This is the test dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
    assert.equal(doveCampaignAccount.createdDate.toNumber(), doveCampaignAccount.updateDate.toNumber());
    assert.ok(!doveCampaignAccount.isLocked);
    assert.ok(!doveCampaignAccount.isDeleted);
    assert.equal(doveCampaignAccount.videoLink, "https://youtu.be/zcVfBMse1Uw");
    assert.equal(doveCampaignAccount.amountPooled, 0);
    assert.equal(doveCampaignAccount.amountTransferred, 0);
    assert.equal(doveCampaignAccount.decision, 0);
    assert.ok(equalDateTime(doveCampaignAccount.createdDate.toNumber(), dove_campaign_created_date));
    assert.ok(equalDateTime(doveCampaignAccount.lastDateTransferred.toNumber(), dove_campaign_created_date));

    await sleep();

    // This test case was not intended for the actual potential conflict of the country code, but only test.
    const updatedCampaign = await updateDoveCampaign(
      doveCampaign,
      "https://twitter.com/Ohsugi/status/1616505441705463816?s=20&t=vofTMniwI3ysTx9wyxy8dA",
      "Test Porject 2",
      "Japan",
      "Cayman Islands",
      "This is the updated dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
      "https://www.youtube.com/watch?v=zcVfBMse1Uw&ab_channel=DATALab",
      false,
      program,
      admin,
    );
    const updatedCampaignAccount = await program.account.doveCampaign.fetch(updatedCampaign);
    assert.equal(doveCampaignAccount.adminPubkey.toString(), admin.publicKey.toString());
    assert.equal(updatedCampaignAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1616505441705463816?s=20&t=vofTMniwI3ysTx9wyxy8dA");
    assert.equal(updatedCampaignAccount.campaignName, "Test Porject 2");
    assert.equal(updatedCampaignAccount.targetCountryCode, "JP");
    assert.equal(updatedCampaignAccount.opponentCountryCode, "KY");
    assert.equal(updatedCampaignAccount.description, "This is the updated dove campaign, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
    assert.notEqual(updatedCampaignAccount.createdDate.toNumber(), updatedCampaignAccount.updateDate.toNumber());
    assert.ok(!updatedCampaignAccount.isLocked);
    assert.equal(updatedCampaignAccount.videoLink, "https://www.youtube.com/watch?v=zcVfBMse1Uw&ab_channel=DATALab");
    assert.equal(doveCampaignAccount.amountPooled, 0);
    assert.equal(doveCampaignAccount.amountTransferred, 0);
    assert.equal(doveCampaignAccount.decision, 0);
    assert.ok(equalDateTime(doveCampaignAccount.createdDate.toNumber(), dove_campaign_created_date));
    assert.ok(equalDateTime(doveCampaignAccount.lastDateTransferred.toNumber(), dove_campaign_created_date));
  });
});
