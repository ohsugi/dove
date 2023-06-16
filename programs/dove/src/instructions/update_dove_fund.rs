use crate::{model::{DoveFund, DoveProject, SizeDef}, error::ErrorCode};
use anchor_lang::{prelude::*, accounts::account::Account};

#[derive(Accounts)]
#[instruction(
    new_amount_pooled: u64,
    new_decision: f32,
    new_shows_user: bool,           
    new_shows_pooled_amount: bool,
    new_shows_transferred_amount: bool,
)]

pub struct UpdateDoveFund<'info> {
    #[account(mut)]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateDoveFund>,
    new_amount_pooled: u64,             // The new pooled amount (as Lamports)
    new_decision: f32,                  // The new decision percentage
    new_shows_user: bool,               // New shows_user flag
    new_shows_pooled_amount: bool,      // New shows_pooled_amount flag
    new_shows_transferred_amount: bool, // New shows_transferred_amount flag
) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;
    let user: &mut Signer = &mut ctx.accounts.user;

    require!(
        dove_fund.user_pubkey == user.key(),
        ErrorCode::InvalidUserToUpdateDoveFund
    );

    require!(
        dove_fund.project_pubkey == dove_project.key(),
        ErrorCode::InvalidProjectToUpdateDoveFund
    );

    require!(!dove_project.is_locked, ErrorCode::DoveProjectIsLocked);

    require!(
        new_amount_pooled > DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );
    require!(
        dove_project.amount_pooled + new_amount_pooled <= DoveFund::MAX_AMOUNT_TO_POOLED,
        ErrorCode::TooLargeAmountPooled
    );

    require!(
        new_decision >= DoveFund::MIN_PERCENTAGE,
        ErrorCode::TooSmallDecision
    );
    require!(
        new_decision <= DoveFund::MAX_PERCENTAGE,
        ErrorCode::TooLargeDecision
    );
    require!(
        (dove_fund.amount_pooled != new_amount_pooled) || (dove_fund.decision != new_decision) || (dove_fund.shows_user != new_shows_user) || (dove_fund.shows_pooled_amount != new_shows_pooled_amount) || (dove_fund.shows_transferred_amount != new_shows_transferred_amount),
            ErrorCode::NoChangeToDoveFund
    );

    // If the last update was before the last transaction of the project, the pooled money was transferred.
    if dove_fund.update_date < dove_project.last_date_transferred {
        dove_fund.amount_transferred = dove_fund.amount_pooled;
    }

    // Add the amount to the DoveFund
    if dove_fund.amount_pooled < new_amount_pooled {
        // Transfer Solana to dove_fund_account from the user wallet
        let added_amount = new_amount_pooled - dove_fund.amount_pooled;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &user.key(),
            &dove_fund.key(),
            added_amount
            );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                user.to_account_info(),
                dove_fund.to_account_info(),
            ]
        )?;

    // Withdraw the amount pooled from the DoveFund
    } else if dove_fund.amount_pooled > new_amount_pooled {
        // Transfer Solana to the user wallet from dove_project_account
        let withdraw_amount = dove_fund.amount_pooled - new_amount_pooled;
        // Transfer Solana to the user wallet from dove_fund_account
        let rent_balance = Rent::get()?.minimum_balance(dove_fund.to_account_info().data_len());
        require!(**dove_fund.to_account_info().lamports.borrow() - rent_balance >= withdraw_amount, ErrorCode::InsufficientFundsInDoveFund);
        **dove_fund.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;
        **user.to_account_info().try_borrow_mut_lamports()? += withdraw_amount;
    }

    let old_amount_pooled: u64 = dove_fund.amount_pooled;
    dove_fund.amount_pooled = new_amount_pooled;
    let old_decision: f32 = dove_fund.decision;
    dove_fund.decision = new_decision;

    dove_fund.shows_user = new_shows_user;
    dove_fund.shows_pooled_amount = new_shows_pooled_amount;
    dove_fund.shows_transferred_amount = new_shows_transferred_amount;
    dove_fund.update_date = DoveFund::get_now_as_unix_time();

    // Update DoveProject
    let old_project_amount_pooled: u64 = dove_project.amount_pooled;
    let old_project_decision:f32 = dove_project.decision;

    dove_project.amount_pooled = dove_project.amount_pooled - old_amount_pooled + new_amount_pooled;
    dove_project.decision = (old_project_amount_pooled as f32 * old_project_decision - old_amount_pooled as f32 * old_decision + new_amount_pooled as f32 * new_decision) / (old_project_amount_pooled - old_amount_pooled + new_amount_pooled) as f32;
    dove_project.update_date = dove_fund.update_date;
    Ok(())
}
