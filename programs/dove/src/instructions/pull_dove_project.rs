use crate::{
    error::ErrorCode,
    model::{DoveProject, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    checked_amount_pooled: u64, // The checked pooled amount (as Lamports)
)]

pub struct PullDoveProject<'info> {
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account()]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<PullDoveProject>,
    checked_amount_pooled: u64, // The checked pooled amount (as Lamports)
) -> Result<()> {
    let project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        project.admin_pubkey == ctx.accounts.admin.key(),
        ErrorCode::InvalidUser
    );
    require!(!project.is_deleted, ErrorCode::DoveProjectIsAlreadyDeleted);
    require!(!project.is_locked, ErrorCode::DoveProjectIsLocked);

    // If the checked values are inconsistent with the calculated values with the fetched from all DoveFund PDAs.
    require!(
        DoveProject::almost_equal_amount_pooled(project.amount_pooled, checked_amount_pooled),
        ErrorCode::InconsistentAmountPooled
    );

    require!(
        project.decision >= DoveProject::DECISION_THRESHOLD,
        ErrorCode::PullFundsIsNotAllowed
    );

    // lock any changes regarding the project even for the DoveFund except pulling the pooled amount.
    project.is_locked = true;

    // The last date transferred is recoreded for the upcoming pull instructions to DoveFunds.
    project.update_date = DoveProject::get_now_as_unix_time();
    project.last_date_transferred = project.update_date;

    Ok(())
}
