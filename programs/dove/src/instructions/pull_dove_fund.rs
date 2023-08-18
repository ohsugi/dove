use crate::{
    error::ErrorCode,
    model::{DoveCampaign, DoveFund, SizeDef},
};
use anchor_lang::{accounts::account::Account, prelude::*};

#[derive(Accounts)]
#[instruction()]

pub struct PullDoveFund<'info> {
    #[account(mut)]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PullDoveFund>) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_campaign: &mut Account<DoveCampaign> = &mut ctx.accounts.dove_campaign;
    let admin: &mut Signer = &mut ctx.accounts.admin;

    require!(
        dove_fund.campaign_pubkey == dove_campaign.key(),
        ErrorCode::InvalidCampaign
    );

    require!(
        dove_campaign.admin_pubkey == admin.key(),
        ErrorCode::InvalidUser
    );

    require!(
        dove_campaign.decision >= DoveCampaign::DECISION_THRESHOLD,
        ErrorCode::PullFundsIsNotAllowed
    );

    require!(
        dove_fund.amount_pooled >= DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );

    require!(dove_campaign.is_locked, ErrorCode::DoveCampaignIsNotLocked);

    require!(
        dove_fund.update_date < dove_campaign.last_date_transferred,
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
    dove_campaign.update_date = now_date;
    dove_campaign.amount_pooled -= pooled_amount;
    dove_campaign.amount_transferred += pooled_amount;

    if dove_campaign.amount_pooled == 0 {
        dove_campaign.is_locked = false;
    }
    Ok(())
}
