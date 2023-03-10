import * as anchor from "@project-serum/anchor";
import { Program, web3, utils } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";

export const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

export const stringToBytes = (str: string) => {
	return utils.bytes.utf8.encode(str);
};

export const findAddress = async (seeds: (Uint8Array | Buffer)[]):
	Promise<[web3.PublicKey, number]> => {
	const program = anchor.workspace.Dove as Program<Dove>;
	return await web3.PublicKey.findProgramAddressSync(seeds, program.programId);
};

export function loadWalletKey(keypairFile: string): web3.Keypair {
	const fs = require("fs");
	const loaded = web3.Keypair.fromSecretKey(
		new Uint8Array(JSON.parse(fs.readFileSync(keypairFile).toString())),
	);
	return loaded;
};

export const createUser = async (
	program: Program<Dove>,
	default_lamports: number,
): Promise<web3.Keypair> => {
	const user = web3.Keypair.generate();
	const connection = program.provider.connection;
	const signature = await connection.requestAirdrop(user.publicKey, default_lamports);
	const { lastValidBlockHeight, blockhash } = await connection.getLatestBlockhash();
	await connection.confirmTransaction({ lastValidBlockHeight, blockhash, signature });
	return user;
};

export const createDoveUser = async (
	user_name: string,
	social_link: string,
	evidence_link: string,
	program: Program<Dove>,
	wallet: anchor.web3.Keypair
): Promise<anchor.web3.PublicKey> => {
	const [doveUser, _] = await findAddress([stringToBytes("dove_user"), stringToBytes(user_name)]);
	await program.methods.createDoveUser(
		user_name,
		social_link,
		evidence_link)
		.accounts({ doveUser, admin: wallet.publicKey }).signers([wallet]).rpc();
	return doveUser;
}

export const getBalance = async (
	program: Program<Dove>,
	wallet: web3.PublicKey,
): Promise<number> => {
	//    return await program.provider.connection.getBalance(wallet);
	return (await program.account.doveProject.getAccountInfo(wallet)).lamports
};

export const createDoveProject = async (
	evidence_link: string,
	project_name: string,
	target_country_name: string,
	opponent_country_name: string,
	description: string,
	video_link: string,
	program: Program<Dove>,
	admin: web3.Keypair,
): Promise<web3.PublicKey> => {
	const [doveProject, _] = await findAddress(
		[
			stringToBytes("dove_project"),
			admin.publicKey.toBuffer(),
			stringToBytes(project_name),
		]);

	await program.methods.createDoveProject(
		evidence_link,
		project_name,
		target_country_name,
		opponent_country_name,
		description,
		video_link,
	).accounts({
		doveProject,
		admin: admin.publicKey,
	}).signers([admin]).rpc();
	return doveProject;
};


export const updateDoveProject = async (
	doveProject: web3.PublicKey,
	evidence_link: string,
	project_name: string,
	target_country_name: string,
	opponent_country_name: string,
	description: string,
	video_link: string,
	is_effective: boolean,
	program: Program<Dove>,
	admin: web3.Keypair,
): Promise<web3.PublicKey> => {
	await program.methods.updateDoveProject(
		evidence_link,
		project_name,
		target_country_name,
		opponent_country_name,
		description,
		video_link,
		is_effective,
	).accounts({
		doveProject,
		admin: admin.publicKey,
	}).signers([admin]).rpc();

	return doveProject;
};

export const createDoveFund = async (
	doveProject: web3.PublicKey,
	amount_pooled: number,
	decision: number,
	shows_user: boolean,
	shows_pooled_amount: boolean,
	shows_transferred_amount: boolean,
	program: Program<Dove>,
	user: web3.Keypair,
): Promise<web3.PublicKey> => {
	const [doveFund, __] = await findAddress(
		[
			stringToBytes("dove_fund"),
			doveProject.toBuffer(),
			user.publicKey.toBuffer(),
		]);

	await program.methods.createDoveFund(
		amount_pooled,
		decision,
		shows_user,
		shows_pooled_amount,
		shows_transferred_amount,
	).accounts({
		doveFund,
		doveProject,
		user: user.publicKey,
	}).signers([user]).rpc();
	return doveFund;
};
