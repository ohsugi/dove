use crate::model::{DoveFund, DoveProject, SizeDef};
use anchor_lang::{prelude::*, accounts::account::Account};
/* 
use solana_client::{
    rpc_client::RpcClient, 
    rpc_filter::{RpcFilterType, Memcmp, MemcmpEncodedBytes, MemcmpEncoding},
    rpc_config::RpcProgramAccountsConfig,
};
use solana_sdk::{commitment_config::CommitmentConfig, bs58};
*/

#[derive(Accounts)]
#[instruction(
    amount_pooled: u64,
    decision: f32,
    shows_user: bool,           
    shows_pooled_amount: bool,
    shows_transferred_amount: bool,
)]
pub struct CreateDoveFund<'info> {
    #[account(init,
      payer=user,
      space=DoveFund::SIZE,
      seeds=[b"dove_fund", dove_project.key().as_ref(), user.key().as_ref()],
      bump,
    )]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateDoveFund>,
    amount_pooled: u64,             // The current pooled amount (as Lamports)
    decision: f32,                  // The decision percentage
    shows_user: bool,                 // If the user will be shown on the project webpage
    shows_pooled_amount: bool,      // If the user's pooled amount on the project webpage
    shows_transferred_amount: bool, // If the user's transferred amount on the project webpage
) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        amount_pooled > DoveFund::MIN_AMOUNT_TO_POOLED,
        TooSmallAmountPooled
    );
    require!(
        amount_pooled <= DoveFund::MAX_AMOUNT_TO_POOLED,
        TooLargeAmountPooled
    );

    require!(
        decision >= DoveFund::MIN_PERCENTAGE,
        TooSmallDecision
    );
    require!(
        decision <= DoveFund::MAX_PERCENTAGE,
        TooLargeDecision
    );

    // Transfer Solana to dove_project_account from the user wallet
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &dove_project.key(),
        amount_pooled
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            dove_project.clone().to_account_info()
        ]
    );

    dove_fund.amount_pooled = amount_pooled;
    dove_fund.amount_transferred = 0;        

    dove_fund.project_pubkey = dove_project.key();
    dove_fund.user_pubkey = ctx.accounts.user.key();
    dove_fund.decision = decision;
    dove_fund.shows_user = shows_user;
    dove_fund.shows_pooled_amount = shows_pooled_amount;
    dove_fund.shows_transferred_amount = shows_transferred_amount;
    dove_fund.created_date = DoveFund::get_now_as_unix_time();
    dove_fund.update_date = dove_fund.created_date;
    dove_fund.bump = *ctx.bumps.get("dove_fund").unwrap();

    // Update DoveProject
    let current_amount_pooled: u64 = dove_project.amount_pooled;
    let current_desicion: f32 = dove_project.decision;
    let new_amount_pooled: u64 = current_amount_pooled + amount_pooled;
    let new_desicion: f32 = (current_amount_pooled as f32 * current_desicion + amount_pooled as f32 * decision) / new_amount_pooled as f32;

    dove_project.amount_pooled = new_amount_pooled;
    dove_project.decision = new_desicion;

    // If the new_decision is more than 0.5, it will trigger transfer the pooled money to the admin wallet.
    if new_desicion >= 0.5 {    
        // Trigger the all fund's transfer!!
        /*
        let connection = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());
        let filters = Some(vec![    //  Query to filter the  DoveFund accounts for the DoveProject
            RpcFilterType::Memcmp(Memcmp {
                offset: 0,
                bytes: MemcmpEncodedBytes::Base58(
                    bs58::encode(dove_fund.project_pubkey).into_string(),   // Match with ProjectPubkey
                ),
                encoding: Some(MemcmpEncoding::Binary),
            }),
            RpcFilterType::DataSize(DoveFund::SIZE as u64),    // Match with DoveFund Size
        ]);

        let accounts = connection.get_program_accounts_with_config(
            &PROGRAM_ID,
            RpcProgramAccountsConfig {
                filters,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(connection.commitment()),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        ).unwrap();

        for (account_pubkey, account) in accounts {
            // transfer fund
            let retrievald_dove_fund: &mut Account<DoveFund> = &mut account;
            account.amount_transferred += retrievald_dove_fund.amount_pooled;
            account.amount_pooled = 0;
        }

        // Transfer Solana to admin_wallet from dove_project_account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &dove_project.key(),
            &dove_project.admin_wallet.key(),
            amount_pooled
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                dove_project.clone().to_account_info()
            ]
        );
        dove_project.amount_pooled = new_amount_pooled;
        dove_project.decision = new_desicion;
        */
    }

    Ok(())
}
