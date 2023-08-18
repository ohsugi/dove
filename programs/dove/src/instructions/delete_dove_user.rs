use crate::{error::ErrorCode, model::DoveUser};
use anchor_lang::{accounts::account::Account, prelude::*};

#[derive(Accounts)]
#[instruction()]

pub struct DeleteDoveUser<'info> {
    #[account(mut, close = user)]
    pub dove_user: Account<'info, DoveUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeleteDoveUser>) -> Result<()> {
    let dove_user: &mut Account<DoveUser> = &mut ctx.accounts.dove_user;
    let user: &mut Signer = &mut ctx.accounts.user;

    require!(dove_user.user_pubkey == user.key(), ErrorCode::InvalidUser);

    // Deleting DoveUser will only removes the information for that user and pulls back the stored Lamports to the user's wallet, and no direct effect on DoveCampaign or DoveFund.
    Ok(())
}
