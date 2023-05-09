use crate::{
    error::ErrorCode,
    model::{DoveFund, DoveProject, SizeDef},
};
use anchor_lang::{accounts::account::Account, prelude::*, solana_program::program::invoke_signed};

#[derive(Accounts)]
#[instruction()]

pub struct PullDoveFund<'info> {
    #[account(mut)]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PullDoveFund>) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;
    let admin: &mut Signer = &mut ctx.accounts.admin;

    require!(
        dove_fund.project_pubkey == dove_project.key(),
        ErrorCode::InvalidProjectToPullDoveFund
    );

    require!(
        dove_project.admin_pubkey == admin.key(),
        ErrorCode::InvalidAdminToPullDoveFund
    );

    require!(
        dove_project.decision >= DoveProject::DECISION_THRESHOLD,
        ErrorCode::PullDoveFundIsNotAllowed
    );

    require!(
        dove_fund.amount_pooled >= DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );

    require!(
        dove_project.is_locked,
        ErrorCode::DoveProjectIsNotLockedWhenPullingDoveFund
    );

    require!(
        dove_fund.update_date < dove_project.last_date_transferred,
        ErrorCode::DoveFundWasAlreadyTransferred
    );

    // Transfer Solana to dove_fund_account from the user wallet
    let pooled_amount = dove_fund.amount_pooled;
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &dove_fund.key(),
        &admin.key(),
        pooled_amount,
    );
    invoke_signed(
        &ix,
        &[dove_fund.to_account_info(), admin.to_account_info()],
        &[&[
            b"dove_fund".as_ref(),
            dove_project.key().as_ref(),
            dove_fund.user_pubkey.as_ref(),
            dove_fund.bump.to_be_bytes().as_ref(),
        ]],
    )?;
    dove_fund.amount_transferred += pooled_amount;
    dove_fund.amount_pooled = 0;
    let now_date: i64 = anchor_lang::solana_program::clock::Clock::get()?.unix_timestamp;
    dove_fund.update_date = now_date;
    dove_project.update_date = now_date;
    dove_project.amount_pooled -= pooled_amount;
    dove_project.amount_transferred += pooled_amount;

    if dove_project.amount_pooled == 0 {
        dove_project.is_locked = false;
    }
    Ok(())
}
