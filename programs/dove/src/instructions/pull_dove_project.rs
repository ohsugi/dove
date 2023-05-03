use crate::{
    error::ErrorCode,
    model::{DoveProject, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    checked_amount_pooled: u64, // The checked pooled amount (as Lamports)
    checked_decision: f32,      // The checked decision percentage by fetching all DoveFund
    checked_update_date: i64,   // The checked update date by fetching all DoveFund
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
    checked_decision: f32,      // The checked decision percentage by fetching all DoveFund
    checked_update_date: i64,   // The checked update date by fetching all DoveFund
) -> Result<()> {
    let project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        project.admin_pubkey == ctx.accounts.admin.key(),
        ErrorCode::InvalidUserToPullDoveProject
    );

    require!(!project.is_locked, ErrorCode::DoveProjectIsLocked);

    // If the checked values are inconsistent with the calculated values with the fetched from all DoveFund PDAs.
    require!(
        project.amount_pooled - checked_amount_pooled < DoveProject::ACCEPTABLE_AMOUNT_ERROR,
        ErrorCode::InconsistentAmountPooled
    );

    require!(
        project.decision - checked_decision < DoveProject::ACCEPTABLE_DECISION_ERRROR,
        ErrorCode::InconsistentDecision
    );

    require!(
        project.update_date - checked_update_date < DoveProject::ACCEPTABLE_DATE_ERROR,
        ErrorCode::InconsistentUpdateDate
    );

    require!(
        project.decision >= DoveProject::DECISION_THRESHOLD,
        ErrorCode::PullDoveProjectIsNotAllowed
    );

    // lock any changes regarding the project even for the DoveFund except pulling the pooled amount.
    project.is_locked = true;

    // The last date transferred is recoreded for the upcoming pull instructions to DoveFunds.
    project.update_date = DoveProject::get_now_as_unix_time();
    project.last_date_transferred = project.update_date;

    Ok(())
}
