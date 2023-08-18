use crate::{model::{DoveFund, DoveCampaign, SizeDef}, error::ErrorCode};
use anchor_lang::{prelude::*, accounts::account::Account};

#[derive(Accounts)]
#[instruction(
    amount_pooled: u64,
    decision: f32,
    shows_user: bool,           
    shows_pooled_amount: bool,
    shows_transferred_amount: bool,
)]
pub struct CreateDoveFund<'info> {
    #[account(init,
      payer=user,
      space=DoveFund::SIZE,
      seeds=[b"dove_fund".as_ref(), dove_campaign.key().as_ref(), user.key().as_ref()],
      bump,
    )]
    pub dove_fund: Account<'info, DoveFund>,
    #[account(mut)]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateDoveFund>,
    amount_pooled: u64,             // The pooled amount (as Lamports)
    decision: f32,                  // The decision percentage
    shows_user: bool,               // If the user will be shown on the campaign webpage
    shows_pooled_amount: bool,      // If the user's pooled amount on the campaign webpage
    shows_transferred_amount: bool, // If the user's transferred amount on the campaign webpage
) -> Result<()> {
    let dove_fund: &mut Account<DoveFund> = &mut ctx.accounts.dove_fund;
    let dove_campaign: &mut Account<DoveCampaign> = &mut ctx.accounts.dove_campaign;

    require!(
        !dove_campaign.is_deleted,
        ErrorCode::DoveCampaignIsAlreadyDeleted
    );
    require!(!dove_campaign.is_locked, ErrorCode::DoveCampaignIsLocked);

    require!(
        amount_pooled > DoveFund::MIN_AMOUNT_TO_POOLED,
        ErrorCode::TooSmallAmountPooled
    );
    require!(
        dove_campaign.amount_pooled + amount_pooled <= DoveFund::MAX_AMOUNT_TO_POOLED,
        ErrorCode::TooLargeAmountPooled
    );

    require!(
        decision >= DoveFund::MIN_PERCENTAGE,
        ErrorCode::TooSmallDecision
    );
    require!(
        decision <= DoveFund::MAX_PERCENTAGE,
        ErrorCode::TooLargeDecision
    );

    // Transfer Solana to dove_campaign_account from the user wallet
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &dove_fund.key(),
        amount_pooled
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            dove_fund.to_account_info()
        ]
    )?;

    dove_fund.amount_pooled = amount_pooled;
    dove_fund.amount_transferred = 0;        

    dove_fund.campaign_pubkey = dove_campaign.key();
    dove_fund.user_pubkey = ctx.accounts.user.key();
    dove_fund.decision = decision;
    dove_fund.shows_user = shows_user;
    dove_fund.shows_pooled_amount = shows_pooled_amount;
    dove_fund.shows_transferred_amount = shows_transferred_amount;
    dove_fund.created_date = DoveFund::get_now_as_unix_time();
    dove_fund.update_date = dove_fund.created_date;
    dove_fund.bump = *ctx.bumps.get("dove_fund").unwrap();

    // Update DoveCampaign
    let current_amount_pooled: u64 = dove_campaign.amount_pooled;
    let current_decision:f32 = dove_campaign.decision;
    let new_amount_pooled_in_fund: u64 = amount_pooled;
    let new_decision_in_fund: f32 = decision;

    dove_campaign.amount_pooled = current_amount_pooled + new_amount_pooled_in_fund;
    dove_campaign.decision = (current_amount_pooled as f32 * current_decision
        + new_amount_pooled_in_fund as f32 * new_decision_in_fund)
        / (current_amount_pooled + new_amount_pooled_in_fund) as f32;
    dove_campaign.update_date = DoveCampaign::get_now_as_unix_time();
    Ok(())
}
