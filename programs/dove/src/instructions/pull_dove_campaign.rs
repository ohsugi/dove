use crate::{
    error::ErrorCode,
    model::{DoveCampaign, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    checked_amount_pooled: u64, // The checked pooled amount (as Lamports)
)]

pub struct PullDoveCampaign<'info> {
    #[account(mut)]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account()]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<PullDoveCampaign>,
    checked_amount_pooled: u64, // The checked pooled amount (as Lamports)
) -> Result<()> {
    let campaign: &mut Account<DoveCampaign> = &mut ctx.accounts.dove_campaign;

    require!(
        campaign.admin_pubkey == ctx.accounts.admin.key(),
        ErrorCode::InvalidUser
    );
    require!(
        !campaign.is_deleted,
        ErrorCode::DoveCampaignIsAlreadyDeleted
    );
    require!(!campaign.is_locked, ErrorCode::DoveCampaignIsLocked);

    // If the checked values are inconsistent with the calculated values with the fetched from all DoveFund PDAs.
    require!(
        DoveCampaign::almost_equal_amount_pooled(campaign.amount_pooled, checked_amount_pooled),
        ErrorCode::InconsistentAmountPooled
    );

    require!(
        campaign.decision >= DoveCampaign::DECISION_THRESHOLD,
        ErrorCode::PullFundsIsNotAllowed
    );

    // lock any changes regarding the campaign even for the DoveFund except pulling the pooled amount.
    campaign.is_locked = true;

    // The last date transferred is recoreded for the upcoming pull instructions to DoveFunds.
    campaign.update_date = DoveCampaign::get_now_as_unix_time();
    campaign.last_date_transferred = campaign.update_date;

    Ok(())
}
