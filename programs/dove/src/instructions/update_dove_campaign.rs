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
    is_locked: bool,
)]
pub struct UpdateDoveCampaign<'info> {
    #[account(mut)]
    pub dove_campaign: Account<'info, DoveCampaign>,
    #[account()]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateDoveCampaign>,
    evidence_link: String,
    campaign_name: String,
    target_country_name: String,
    opponent_country_name: String,
    description: String,
    video_link: String,
    is_locked: bool,
) -> Result<()> {
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
            || (opponent_country != None), // otherwise, it should be different from the target country
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

    require!(
        (campaign.evidence_link != evidence_link)
            || (campaign.campaign_name != campaign_name)
            || (campaign.target_country_code != target_country.unwrap().to_string())
            || (campaign.opponent_country_code != opponent_country.unwrap().to_string())
            || (campaign.description != description)
            || (campaign.video_link != video_link)
            || (campaign.is_locked != is_locked),
        ErrorCode::NoUpdateApplied,
    );

    campaign.evidence_link = evidence_link;
    campaign.campaign_name = campaign_name;
    campaign.target_country_code = target_country.unwrap().to_string();
    if opponent_country_name.as_str() == "" {
        campaign.opponent_country_code = "".to_string();
    } else {
        campaign.opponent_country_code = opponent_country.unwrap().to_string();
    }
    campaign.description = description;

    campaign.update_date = DoveCampaign::get_now_as_unix_time();
    campaign.is_locked = is_locked;
    campaign.video_link = video_link;
    Ok(())
}
