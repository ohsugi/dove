use crate::{
    error::ErrorCode,
    model::{DoveCampaign, SizeDef},
};
use anchor_lang::prelude::*;

use iso_country::Country;

#[derive(Accounts)]
#[instruction(
    evidence_link: String,
    campaign_name: String,
    target_country_name: String,
    opponent_country_name: String,
    description: String,
    video_link: String,
)]
pub struct CreateDoveCampaign<'info> {
    #[account(init,
      payer=admin,
      space=DoveCampaign::SIZE,
      seeds=[b"dove_campaign".as_ref(), admin.key().as_ref(), campaign_name.as_bytes()],
      bump,
    )]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateDoveCampaign>,
    evidence_link: String,         // HTML link to prove Admin's identity
    campaign_name: String,         // Campaign Name
    target_country_name: String,   // Target country name
    opponent_country_name: String, // Opponent country name
    description: String,           // Description and the usage of the transferred Solana
    video_link: String,            // Video link to describe the campaign (intended youtube link)
) -> Result<()> {
    let campaign: &mut Account<DoveCampaign> = &mut ctx.accounts.dove_campaign;

    require!(
        evidence_link.len() <= DoveCampaign::MAX_HYPERLINK,
        ErrorCode::TooLongEvidenceLink
    );

    require!(
        campaign_name.len() >= DoveCampaign::MIN_CAMPAIGN_NAME,
        ErrorCode::TooShortCampaignName
    );
    require!(
        campaign_name.len() <= DoveCampaign::MAX_CAMPAIGN_NAME,
        ErrorCode::TooLongCampaignName
    );

    let target_country: Option<Country> = Country::from_name(target_country_name.as_str());
    require!(target_country != None, ErrorCode::InvalidTargetCountryName);

    let opponent_country: Option<Country> = Country::from_name(opponent_country_name.as_str());
    require!(
        (opponent_country_name.as_str() == "")    // opponent_country_name can be empty 
            || (opponent_country != None), // otherwise, it should be not None
        ErrorCode::InvalidOpponentCountryName
    );

    require!(
        (opponent_country_name.as_str() == "")    // opponent_country_name can be empty 
            || (target_country.unwrap() != opponent_country.unwrap()), // otherwise, it should be different from the target country
        ErrorCode::TargetAndOpponentCountriesAreSame
    );

    require!(
        description.len() >= DoveCampaign::MIN_DESCRIPTION,
        ErrorCode::TooShortDescription
    );
    require!(
        description.len() <= DoveCampaign::MAX_DESCRIPTION,
        ErrorCode::TooLongDescription
    );

    require!(
        video_link.len() <= DoveCampaign::MAX_HYPERLINK,
        ErrorCode::TooLongVideoLink
    );

    campaign.admin_pubkey = *ctx.accounts.admin.key;
    campaign.evidence_link = evidence_link;
    campaign.campaign_name = campaign_name;
    campaign.target_country_code = target_country.unwrap().to_string();
    if opponent_country_name.as_str() == "" {
        campaign.opponent_country_code = "".to_string();
    } else {
        campaign.opponent_country_code = opponent_country.unwrap().to_string();
    }
    campaign.description = description;

    campaign.created_date = DoveCampaign::get_now_as_unix_time();
    campaign.update_date = campaign.created_date;
    campaign.is_locked = false;
    campaign.is_deleted = false;
    campaign.video_link = video_link;
    campaign.amount_pooled = 0;
    campaign.amount_transferred = 0;
    campaign.last_date_transferred = campaign.created_date;
    campaign.bump = *ctx.bumps.get("dove_campaign").unwrap();
    Ok(())
}
