use crate::{
    error::ErrorCode,
    model::{DoveUser, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    user_name: String,
    social_media_link: String,
    evidence_link: String,
    is_shown: bool,
)]

pub struct UpdateDoveUser<'info> {
    #[account(mut)]
    pub dove_user: Account<'info, DoveUser>,
    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateDoveUser>,
    user_name: String,
    social_media_link: String,
    evidence_link: String,
    is_shown: bool,
) -> Result<()> {
    let dove_user: &mut Account<DoveUser> = &mut ctx.accounts.dove_user;

    require!(
        user_name.len() >= DoveUser::MIN_USER_NAME,
        ErrorCode::TooShortUserName
    );
    require!(
        user_name.len() <= DoveUser::MAX_USER_NAME,
        ErrorCode::TooLongUserName
    );

    require!(
        social_media_link.len() >= DoveUser::MIN_HYPERLINK,
        ErrorCode::TooShortUrl
    );
    require!(
        social_media_link.len() <= DoveUser::MAX_HYPERLINK,
        ErrorCode::TooLongUrl
    );

    require!(
        evidence_link.len() >= DoveUser::MIN_HYPERLINK,
        ErrorCode::TooShortUrl
    );
    require!(
        evidence_link.len() <= DoveUser::MAX_HYPERLINK,
        ErrorCode::TooLongUrl
    );

    dove_user.user_name = user_name;
    dove_user.social_media_link = social_media_link;
    dove_user.evidence_link = evidence_link;
    dove_user.is_shown = is_shown;
    dove_user.update_date = DoveUser::get_now_as_unix_time();
    Ok(())
}
