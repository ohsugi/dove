use crate::{
    error::ErrorCode,
    model::{DoveUser, SizeDef},
};
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

    require!(
        dove_user.user_pubkey == user.key(),
        ErrorCode::InvalidUserToDeleteDoveUser
    );

    require!(
        dove_user.amount_pooled >= DoveUser::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );

    // Deleting DoveUser will only removes the information for that user and pulls back the stored Lamports to the user's wallet, and no direct effect on DoveProject or DoveFund.
    let all_amount: u64 = **dove_user.to_account_info().lamports.borrow();
    require!(
        all_amount > DoveUser::MIN_AMOUNT_TO_POOLED,
        ErrorCode::InsufficientFundsInDoveUser
    );
    **dove_user.to_account_info().try_borrow_mut_lamports()? -= all_amount;
    **user.to_account_info().try_borrow_mut_lamports()? += all_amount;
    Ok(())
}