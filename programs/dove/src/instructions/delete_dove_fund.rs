use crate::{
    error::ErrorCode,
    model::{DoveCampaign, DoveFund, SizeDef},
};
use anchor_lang::{accounts::account::Account, prelude::*};

#[derive(Accounts)]
#[instruction()]

pub struct DeleteDoveFund<'info> {
    #[account(mut, close = user)]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteDoveFund>) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_campaign: &mut Account<DoveCampaign> = &mut ctx.accounts.dove_campaign;
    let user: &mut Signer = &mut ctx.accounts.user;

    require!(dove_fund.user_pubkey == user.key(), ErrorCode::InvalidUser);
    require!(
        dove_fund.campaign_pubkey == dove_campaign.key(),
        ErrorCode::InvalidCampaign
    );
    require!(!dove_campaign.is_locked, ErrorCode::DoveCampaignIsLocked);
    require!(
        dove_fund.amount_pooled >= DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );
    require!(
        dove_campaign.amount_pooled - dove_fund.amount_pooled >= DoveCampaign::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );

    // If the last update was before the last transaction of the campaign, the pooled money was transferred.
    if dove_fund.update_date < dove_campaign.last_date_transferred {
        dove_fund.amount_transferred = dove_fund.amount_pooled;
    }

    let old_amount_pooled: u64 = dove_fund.amount_pooled;
    dove_fund.amount_pooled = 0;
    let old_decision: f32 = dove_fund.decision;

    // Update DoveCampaign
    let old_campaign_amount_pooled: u64 = dove_campaign.amount_pooled;
    let old_campaign_decision: f32 = dove_campaign.decision;

    dove_campaign.amount_pooled = dove_campaign.amount_pooled - old_amount_pooled;
    if dove_campaign.amount_pooled == 0 {
        dove_campaign.decision = 0.5;
    } else {
        dove_campaign.decision = (old_campaign_amount_pooled as f32 * old_campaign_decision
            - old_amount_pooled as f32 * old_decision)
            / dove_campaign.amount_pooled as f32;
    }
    dove_campaign.update_date = DoveCampaign::get_now_as_unix_time();
    Ok(())
}
