use crate::{
    error::ErrorCode,
    model::{DoveCampaign, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct DeleteDoveCampaign<'info> {
    #[account(mut)]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account()]
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteDoveCampaign>) -> Result<()> {
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
    campaign.update_date = DoveCampaign::get_now_as_unix_time();
    campaign.is_deleted = true;
    Ok(())
}
