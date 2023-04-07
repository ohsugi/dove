use crate::{
    error::ErrorCode,
    model::{DoveUser, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(user_name: String)]
pub struct CreateDoveUser<'info> {
    #[account(init,
      payer=user,
      space=DoveUser::SIZE,
      seeds=[b"dove_user".as_ref(), user.key().as_ref(), user_name.as_bytes()],
      bump,
    )]
    pub dove_user: Account<'info, DoveUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateDoveUser>,
    user_name: String,
    social_media_link: String,
    evidence_link: String,
    is_shown: bool,
) -> Result<()> {
    let dove_user: &mut Account<DoveUser> = &mut ctx.accounts.dove_user;
    let user: &Signer = &ctx.accounts.user;

    require!(
        user_name.len() >= DoveUser::MIN_USER_NAME,
        ErrorCode::TooShortUserName
    );

    require!(
        user_name.len() <= DoveUser::MAX_USER_NAME,
        ErrorCode::TooLongUserName
    );

    require!(social_media_link.len() >= 4, ErrorCode::TooShortUrl);
    require!(
        social_media_link.len() <= DoveUser::MAX_HYPERLINK,
        ErrorCode::TooLongUrl
    );

    require!(evidence_link.len() >= 4, ErrorCode::TooShortUrl);
    require!(
        evidence_link.len() <= DoveUser::MAX_HYPERLINK,
        ErrorCode::TooLongUrl
    );

    dove_user.user_name = user_name;
    dove_user.social_media_link = social_media_link;
    dove_user.evidence_link = evidence_link;
    dove_user.is_shown = is_shown;
    dove_user.amount_pooled = 0;
    dove_user.amount_transferred = 0;
    dove_user.created_date = DoveUser::get_now_as_unix_time();
    dove_user.update_date = dove_user.created_date;
    dove_user.bump = *ctx.bumps.get("dove_user").unwrap();
    Ok(())
}
