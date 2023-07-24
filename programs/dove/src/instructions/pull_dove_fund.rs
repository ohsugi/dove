use crate::{
    error::ErrorCode,
    model::{DoveFund, DoveProject, SizeDef},
};
use anchor_lang::{accounts::account::Account, prelude::*};

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
        ErrorCode::InvalidProject
    );

    require!(
        dove_project.admin_pubkey == admin.key(),
        ErrorCode::InvalidUser
    );

    require!(
        dove_project.decision >= DoveProject::DECISION_THRESHOLD,
        ErrorCode::PullFundsIsNotAllowed
    );

    require!(
        dove_fund.amount_pooled >= DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );

    require!(dove_project.is_locked, ErrorCode::DoveProjectIsNotLocked);

    require!(
        dove_fund.update_date < dove_project.last_date_transferred,
        ErrorCode::DoveFundWasAlreadyTransferred
    );

    // Transfer Solana to dove_fund_account from the admin wallet
    let pooled_amount = dove_fund.amount_pooled;
    **dove_fund.to_account_info().try_borrow_mut_lamports()? -= pooled_amount;
    **admin.to_account_info().try_borrow_mut_lamports()? += pooled_amount;

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
