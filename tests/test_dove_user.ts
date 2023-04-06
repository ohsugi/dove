import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";
import { createUser, createDoveUser } from "./util";

import assert from 'assert';

describe("test adding users", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());
	const program = anchor.workspace.Dove as Program<Dove>;
	const DEFAULT_LAMPORTS: number = 4 * anchor.web3.LAMPORTS_PER_SOL;
	let admin: anchor.web3.Keypair;

	before(async () => {
		admin = await createUser(program, DEFAULT_LAMPORTS);
	})

	it("creates a dove user", async () => {
		const doveUser = await createDoveUser(
			"admin",
			"social_link",
			"evidence",
			program,
			admin,
		);
		const doveUserAccount = await program.account.doveUser.fetch(doveUser);
		assert.equal(doveUserAccount.userName, "admin");
		assert.equal(doveUserAccount.isShown, false);
		assert.equal(doveUserAccount.amountPooled, 0);
		assert.equal(doveUserAccount.amountTransferred, 0);
	});
});
