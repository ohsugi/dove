use crate::{
    error::ErrorCode,
    model::{DoveFund, DoveProject, SizeDef},
};
use anchor_lang::{accounts::account::Account, prelude::*};

#[derive(Accounts)]
#[instruction()]

pub struct DeleteDoveFund<'info> {
    #[account(mut, close = user)]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteDoveFund>) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;
    let user: &mut Signer = &mut ctx.accounts.user;

    require!(
        dove_fund.user_pubkey == user.key(),
        ErrorCode::InvalidUserToDoveFund
    );

    require!(
        dove_fund.amount_pooled > DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );
    require!(
        dove_project.amount_pooled - dove_fund.amount_pooled >= DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );

    // If the last update was before the last transaction of the project, the pooled money was transferred.
    if dove_fund.update_date < dove_project.last_date_transferred {
        dove_fund.amount_transferred = dove_fund.amount_pooled;
    }

    let all_amount: u64 = **dove_fund.to_account_info().lamports.borrow();
    require!(
        all_amount > DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::InsufficientFundsInDoveFund
    );
    **dove_fund.to_account_info().try_borrow_mut_lamports()? -= all_amount;
    **user.to_account_info().try_borrow_mut_lamports()? += all_amount;

    let old_amount_pooled: u64 = dove_fund.amount_pooled;
    dove_fund.amount_pooled = 0;
    let old_decision: f32 = dove_fund.decision;

    // Update DoveProject
    let old_project_amount_pooled: u64 = dove_project.amount_pooled;
    let old_project_decision: f32 = dove_project.decision;

    dove_project.amount_pooled = dove_project.amount_pooled - old_amount_pooled;
    if dove_project.amount_pooled == 0 {
        dove_project.decision = 0.5;
    } else {
        dove_project.decision = (old_project_amount_pooled as f32 * old_project_decision
            - old_amount_pooled as f32 * old_decision)
            / dove_project.amount_pooled as f32;
    }
    dove_project.update_date = DoveProject::get_now_as_unix_time();
    Ok(())
}
