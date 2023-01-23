import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveProject, updateDoveProject, sleep } from "./util";

import assert from 'assert';

describe("dove", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Dove as Program<Dove>;
  let admin: anchor.web3.Keypair;

  before(async () => {
    admin = await createUser(program);
  })

  it("createDoveProject", async () => {
    const doveProject = await createDoveProject(
      "Test Guy 0",
      "",
      "Test Porject 0",
      "Taiwan, Province of China[a]",
      "China",
      "This is the test dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
      "",
      program,
      admin,
    );
    const doveProjectAccount = await program.account.doveProject.fetch(doveProject);
    assert.equal(doveProjectAccount.adminName, "Test Guy 0");
    assert.equal(doveProjectAccount.evidenceLink, "");
    assert.equal(doveProjectAccount.projectName, "Test Porject 0");
    assert.equal(doveProjectAccount.targetCountryCode, "TW");
    assert.equal(doveProjectAccount.opponentCountryCode, "CN");
    assert.equal(doveProjectAccount.description, "This is the test dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
    assert.equal(doveProjectAccount.createdDate.toNumber(), doveProjectAccount.updateDate.toNumber());
    assert.equal(doveProjectAccount.isEffective, true);
    assert.equal(doveProjectAccount.isDeleted, false);
    assert.equal(doveProjectAccount.videoLink, "");
    assert.equal(doveProjectAccount.amountTransferred, 0.0);
    assert.equal(doveProjectAccount.lastDateFunded.toNumber(), doveProjectAccount.createdDate.toNumber());
    assert.equal(doveProjectAccount.amountPooledTransition.length, 0);
  });

  it("updateDoveProject", async () => {
    const doveProject = await createDoveProject(
      "Test Guy 1",
      "https://twitter.com/Ohsugi/status/1615827817627017217?s=20&t=gFmtF8G4VrnDrzB0jhCsRA",
      "Test Porject 1",
      "Japan",
      "",
      "This is the test dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
      "https://youtu.be/zcVfBMse1Uw",
      program,
      admin,
    );
    const doveProjectAccount = await program.account.doveProject.fetch(doveProject);
    assert.equal(doveProjectAccount.adminName, "Test Guy 1");
    assert.equal(doveProjectAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1615827817627017217?s=20&t=gFmtF8G4VrnDrzB0jhCsRA");
    assert.equal(doveProjectAccount.projectName, "Test Porject 1");
    assert.equal(doveProjectAccount.targetCountryCode, "JP");
    assert.equal(doveProjectAccount.opponentCountryCode, "");
    assert.equal(doveProjectAccount.description, "This is the test dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
    assert.equal(doveProjectAccount.createdDate.toNumber(), doveProjectAccount.updateDate.toNumber());
    assert.equal(doveProjectAccount.isEffective, true);
    assert.equal(doveProjectAccount.isDeleted, false);
    assert.equal(doveProjectAccount.videoLink, "https://youtu.be/zcVfBMse1Uw");
    assert.equal(doveProjectAccount.amountTransferred, 0.0);
    assert.equal(doveProjectAccount.lastDateFunded.toNumber(), doveProjectAccount.createdDate.toNumber());
    assert.equal(doveProjectAccount.amountPooledTransition.length, 0);

    await sleep(1000);

    const updatedProject = await updateDoveProject(
      doveProject,
      "Test Guy 2",
      "https://twitter.com/Ohsugi/status/1616505441705463816?s=20&t=vofTMniwI3ysTx9wyxy8dA",
      "Test Porject 2",
      "Taiwan, Province of China[a]",
      "China",
      "This is the updated dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!",
      "https://www.youtube.com/watch?v=zcVfBMse1Uw&ab_channel=DATALab",
      false,
      true,
      program,
      admin,
    );
    const updatedProjectAccount = await program.account.doveProject.fetch(updatedProject);
    assert.equal(updatedProjectAccount.adminName, "Test Guy 2");
    assert.equal(updatedProjectAccount.evidenceLink, "https://twitter.com/Ohsugi/status/1616505441705463816?s=20&t=vofTMniwI3ysTx9wyxy8dA");
    assert.equal(updatedProjectAccount.projectName, "Test Porject 2");
    assert.equal(updatedProjectAccount.targetCountryCode, "TW");
    assert.equal(updatedProjectAccount.opponentCountryCode, "CN");
    assert.equal(updatedProjectAccount.description, "This is the updated dove project, and the minimum length of this description should be more than 128, so I need to put more words to go through the test!!");
    assert.notEqual(updatedProjectAccount.createdDate.toNumber(), updatedProjectAccount.updateDate.toNumber());
    assert.equal(updatedProjectAccount.isEffective, false);
    assert.equal(updatedProjectAccount.isDeleted, true);
    assert.equal(updatedProjectAccount.videoLink, "https://www.youtube.com/watch?v=zcVfBMse1Uw&ab_channel=DATALab");
    assert.equal(updatedProjectAccount.amountTransferred, 0.0);
    assert.equal(updatedProjectAccount.lastDateFunded.toNumber(), updatedProjectAccount.createdDate.toNumber());
    assert.notEqual(updatedProjectAccount.lastDateFunded.toNumber(), updatedProjectAccount.updateDate.toNumber());
    assert.equal(updatedProjectAccount.amountPooledTransition.length, 0);
  });
});
