import * as anchor from "@project-serum/anchor";
import { Program, web3, utils } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Dove } from "../target/types/dove";

const ACCEPTABLE_DATE_ERROR = 3000000;
const DEFAULT_WAIT_TIME = 1000;
export const equalDateTime = (dateTime1: number, dateTime2: number): boolean => {
    return Math.abs(dateTime1 - dateTime2) < ACCEPTABLE_DATE_ERROR;
}

export const getNow = (): number => {
    return Math.round(Date.now() / 1000);
};

export const sleep = (ms: number = DEFAULT_WAIT_TIME) => new Promise(r => setTimeout(r, ms));

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
    is_shown: boolean,
    program: Program<Dove>,
    user: web3.Keypair
): Promise<web3.PublicKey> => {
    const [doveUser, _] = await findAddress([
        stringToBytes("dove_user"),
        user.publicKey.toBuffer(),
    ]);
    await program.methods.createDoveUser(
        user_name,
        social_link,
        evidence_link,
        is_shown,
    ).accounts({ doveUser, user: user.publicKey }).signers([user]).rpc();
    return doveUser;
};

export const updateDoveUser = async (
    user_name: string,
    social_link: string,
    evidence_link: string,
    is_shown: boolean,
    program: Program<Dove>,
    user: web3.Keypair
): Promise<web3.PublicKey> => {
    const [doveUser, _] = await findAddress([
        stringToBytes("dove_user"),
        user.publicKey.toBuffer(),
    ]);
    await program.methods.updateDoveUser(
        user_name,
        social_link,
        evidence_link,
        is_shown,
    ).accounts({ doveUser, user: user.publicKey }).signers([user]).rpc();
    return doveUser;
};

export const deleteDoveUser = async (
    program: Program<Dove>,
    user: web3.Keypair
): Promise<web3.PublicKey> => {
    const [doveUser, _] = await findAddress([
        stringToBytes("dove_user"),
        user.publicKey.toBuffer(),
    ]);
    await program.methods.deleteDoveUser(
    ).accounts({
        doveUser,
        user: user.publicKey,
        systemProgram: SystemProgram.programId
    }).signers([user]).rpc();
    return doveUser;
};

export const getBalance = async (
    program: Program<Dove>,
    wallet: web3.PublicKey,
): Promise<number> => {
    return (await program.account.doveCampaign.getAccountInfo(wallet)).lamports
};

export const createDoveCampaign = async (
    evidence_link: string,
    campaign_name: string,
    target_country_name: string,
    opponent_country_name: string,
    description: string,
    video_link: string,
    program: Program<Dove>,
    admin: web3.Keypair,
): Promise<web3.PublicKey> => {
    const [doveCampaign, _] = await findAddress(
        [
            stringToBytes("dove_campaign"),
            admin.publicKey.toBuffer(),
            stringToBytes(campaign_name),
        ]);

    await program.methods.createDoveCampaign(
        evidence_link,
        campaign_name,
        target_country_name,
        opponent_country_name,
        description,
        video_link,
    ).accounts({
        doveCampaign,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId,
    }).signers([admin]).rpc();
    return doveCampaign;
};

export const updateDoveCampaign = async (
    doveCampaign: web3.PublicKey,
    evidence_link: string,
    campaign_name: string,
    target_country_name: string,
    opponent_country_name: string,
    description: string,
    video_link: string,
    is_locked: boolean,
    program: Program<Dove>,
    admin: web3.Keypair,
): Promise<web3.PublicKey> => {
    await program.methods.updateDoveCampaign(
        evidence_link,
        campaign_name,
        target_country_name,
        opponent_country_name,
        description,
        video_link,
        is_locked,
    ).accounts({
        doveCampaign,
        admin: admin.publicKey,
    }).signers([admin]).rpc();
    return doveCampaign;
};

export const pullDoveCampaign = async (
    doveCampaign: web3.PublicKey,
    checked_amount_pooled: number,
    program: Program<Dove>,
    admin: web3.Keypair,
): Promise<web3.PublicKey> => {
    await program.methods.pullDoveCampaign(
        checked_amount_pooled,
    ).accounts({
        doveCampaign,
        admin: admin.publicKey,
    }).signers([admin]).rpc();
    return doveCampaign;
};

export const pullDoveFund = async (
    doveFund: web3.PublicKey,
    doveCampaign: web3.PublicKey,
    program: Program<Dove>,
    admin: web3.Keypair,
): Promise<web3.PublicKey> => {
    await program.methods.pullDoveFund(
    ).accounts({
        doveFund,
        doveCampaign,
        admin: admin.publicKey,
    }).signers([admin]).rpc();
    return doveFund;
};

export const deleteDoveCampaign = async (
    doveCampaign: web3.PublicKey,
    program: Program<Dove>,
    admin: web3.Keypair,
): Promise<web3.PublicKey> => {
    await program.methods.deleteDoveCampaign(
    ).accounts({
        doveCampaign,
        admin: admin.publicKey,
    }).signers([admin]).rpc();
    return doveCampaign;
};

export const createDoveFund = async (
    doveCampaign: web3.PublicKey,
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
            doveCampaign.toBuffer(),
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
        doveCampaign,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
    }).signers([user]).rpc();
    return doveFund;
};

export const updateDoveFund = async (
    doveCampaign: web3.PublicKey,
    new_amount_pooled: number,
    new_decision: number,
    new_shows_user: boolean,
    new_shows_pooled_amount: boolean,
    new_shows_transferred_amount: boolean,
    program: Program<Dove>,
    user: web3.Keypair,
): Promise<web3.PublicKey> => {
    const [doveFund, __] = await findAddress(
        [
            stringToBytes("dove_fund"),
            doveCampaign.toBuffer(),
            user.publicKey.toBuffer(),
        ]);

    await program.methods.updateDoveFund(
        new_amount_pooled,
        new_decision,
        new_shows_user,
        new_shows_pooled_amount,
        new_shows_transferred_amount,
    ).accounts({
        doveFund,
        doveCampaign,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
    }).signers([user]).rpc();
    return doveFund;
};

export const deleteDoveFund = async (
    doveCampaign: web3.PublicKey,
    program: Program<Dove>,
    user: web3.Keypair,
): Promise<web3.PublicKey> => {
    const [doveFund, __] = await findAddress(
        [
            stringToBytes("dove_fund"),
            doveCampaign.toBuffer(),
            user.publicKey.toBuffer(),
        ]);

    await program.methods.deleteDoveFund(
    ).accounts({
        doveFund,
        doveCampaign,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
    }).signers([user]).rpc();
    return doveFund;
};
