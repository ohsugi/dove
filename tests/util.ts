import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dove } from "../target/types/dove";

export const newKeyPair = anchor.web3.Keypair.generate();
export const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

export const stringToBytes = (str: string) => {
    return anchor.utils.bytes.utf8.encode(str);
};

export const findAddress = async (seeds: (Uint8Array | Buffer)[]):
    Promise<[anchor.web3.PublicKey, number]> => {
    const program = anchor.workspace.Dove as Program<Dove>;
    return await anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
};

export function loadWalletKey(keypairFile: string): anchor.web3.Keypair {
    const fs = require("fs");
    const loaded = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(keypairFile).toString())),
    );
    return loaded;
};

export const createUser = async (
    program: Program<Dove>
): Promise<anchor.web3.Keypair> => {
    const user = newKeyPair;
    const connection = program.provider.connection;
    const signature = await connection.requestAirdrop(user.publicKey, 4 * anchor.web3.LAMPORTS_PER_SOL);
    const { lastValidBlockHeight, blockhash } = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ lastValidBlockHeight, blockhash, signature })
    return user;
};

export const createDoveProject = async (
    admin_name: string,
    evidence_link: string,
    project_name: string,
    target_country_name: string,
    opponent_country_name: string,
    description: string,
    video_link: string,
    program: Program<Dove>,
    admin: anchor.web3.Keypair,
): Promise<anchor.web3.PublicKey> => {
    const [doveProject, _] = await findAddress([stringToBytes("dove_project"), stringToBytes(admin_name), stringToBytes(project_name)]);
    await program.methods.createDoveProject(
        admin_name,
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
    doveProject: anchor.web3.PublicKey,
    admin_name: string,
    evidence_link: string,
    project_name: string,
    target_country_name: string,
    opponent_country_name: string,
    description: string,
    video_link: string,
    is_effective: boolean,
    is_deleted: boolean,
    program: Program<Dove>,
    admin: anchor.web3.Keypair,
): Promise<anchor.web3.PublicKey> => {
    await program.methods.updateDoveProject(
        admin_name,
        evidence_link,
        project_name,
        target_country_name,
        opponent_country_name,
        description,
        video_link,
        is_effective,
        is_deleted,
    ).accounts({
        doveProject,
        admin: admin.publicKey,
    }).signers([admin]).rpc();

    return doveProject;
};
