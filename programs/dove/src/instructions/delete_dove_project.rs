use crate::{
    error::ErrorCode,
    model::{DoveProject, SizeDef},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct DeleteDoveProject<'info> {
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account()]
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteDoveProject>) -> Result<()> {
    let project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        project.admin_pubkey == ctx.accounts.admin.key(),
        ErrorCode::InvalidUser
    );
    require!(!project.is_deleted, ErrorCode::DoveProjectIsAlreadyDeleted);
    require!(!project.is_locked, ErrorCode::DoveProjectIsLocked);
    project.update_date = DoveProject::get_now_as_unix_time();
    project.is_deleted = true;
    Ok(())
}
