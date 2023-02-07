use crate::model::{DoveUser, SizeDef};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(user_name: String)]
pub struct CreateDoveUser<'info> {
    #[account(init,
      payer=admin,
      space=DoveUser::SIZE,
      seeds=[b"dove_user", user_name.as_bytes()],
      bump,
    )]
    pub dove_user: Account<'info, DoveUser>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateDoveUser>,
    user_name: String,
    social_link: String,
    evidence_link: String,
) -> Result<()> {
    let user = &mut ctx.accounts.dove_user;
    let admin = &ctx.accounts.admin;

    require!(
        user_name.len() >= DoveUser::MIN_USER_NAME,
        TooShortAdminName
    );

    require!(user_name.len() <= DoveUser::MAX_USER_NAME, TooLongAdminName);

    require!(social_link.len() >= 4, TooShortUrl);
    require!(social_link.len() <= DoveUser::MAX_HYPERLINK, TooLongUrl);

    require!(evidence_link.len() >= 4, TooShortUrl);
    require!(evidence_link.len() <= DoveUser::MAX_HYPERLINK, TooLongUrl);

    user.user_wallet = admin.key();
    user.user_name = user_name;
    user.social_media_link = social_link;
    user.evidence_link = evidence_link;
    user.bump = *ctx.bumps.get("dove_user").unwrap();

    Ok(())
}
