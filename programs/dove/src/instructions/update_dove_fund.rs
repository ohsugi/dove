use crate::{model::{DoveFund, DoveCampaign, SizeDef}, error::ErrorCode};
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
    pub dove_campaign: Account<'info, DoveCampaign>,
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
    let dove_campaign: &mut Account<DoveCampaign> = &mut ctx.accounts.dove_campaign;
    let user: &mut Signer = &mut ctx.accounts.user;

    require!(
        dove_fund.user_pubkey == user.key(),
        ErrorCode::InvalidUser
    );
    require!(
        dove_fund.campaign_pubkey == dove_campaign.key(),
        ErrorCode::InvalidCampaign
    );
    require!(
        !dove_campaign.is_deleted,
        ErrorCode::DoveCampaignIsAlreadyDeleted
    );
    require!(!dove_campaign.is_locked, ErrorCode::DoveCampaignIsLocked);
    require!(
        new_amount_pooled > DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );
    require!(
        dove_campaign.amount_pooled + new_amount_pooled <= DoveFund::MAX_AMOUNT_TO_POOLED,
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
            ErrorCode::NoUpdateApplied
    );

    // If the last update was before the last transaction of the campaign, the pooled money was transferred.
    if dove_fund.update_date < dove_campaign.last_date_transferred {
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
        // Transfer Solana to the user wallet from dove_campaign_account
        let withdraw_amount = dove_fund.amount_pooled - new_amount_pooled;
        // Transfer Solana to the user wallet from dove_fund_account
        let rent_balance = Rent::get()?.minimum_balance(dove_fund.to_account_info().data_len());
        require!(**dove_fund.to_account_info().lamports.borrow() - rent_balance >= withdraw_amount, ErrorCode::InsufficientFunds);
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

    // Update DoveCampaign
    let old_campaign_amount_pooled: u64 = dove_campaign.amount_pooled;
    let old_campaign_decision:f32 = dove_campaign.decision;

    dove_campaign.amount_pooled = dove_campaign.amount_pooled - old_amount_pooled + new_amount_pooled;
    dove_campaign.decision = (old_campaign_amount_pooled as f32 * old_campaign_decision - old_amount_pooled as f32 * old_decision + new_amount_pooled as f32 * new_decision) / (old_campaign_amount_pooled - old_amount_pooled + new_amount_pooled) as f32;
    dove_campaign.update_date = dove_fund.update_date;
    Ok(())
}
