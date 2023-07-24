use crate::{
    error::ErrorCode,
    model::{DoveProject, SizeDef},
};
use anchor_lang::prelude::*;

use iso_country::Country;

#[derive(Accounts)]
#[instruction(
    evidence_link: String,
    project_name: String,
    target_country_name: String,
    opponent_country_name: String,
    description: String,
    video_link: String,
)]
pub struct CreateDoveProject<'info> {
    #[account(init,
      payer=admin,
      space=DoveProject::SIZE,
      seeds=[b"dove_project".as_ref(), admin.key().as_ref(), project_name.as_bytes()],
      bump,
    )]
    pub dove_project: Account<'info, DoveProject>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateDoveProject>,
    evidence_link: String,         // HTML link to prove Admin's identity
    project_name: String,          // Project Name
    target_country_name: String,   // Target country name
    opponent_country_name: String, // Opponent country name
    description: String,           // Description and the usage of the transferred Solana
    video_link: String,            // Video link to describe the project (intended youtube link)
) -> Result<()> {
    let project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        evidence_link.len() <= DoveProject::MAX_HYPERLINK,
        ErrorCode::TooLongEvidenceLink
    );

    require!(
        project_name.len() >= DoveProject::MIN_PROJECT_NAME,
        ErrorCode::TooShortProjectName
    );
    require!(
        project_name.len() <= DoveProject::MAX_PROJECT_NAME,
        ErrorCode::TooLongProjectName
    );

    let target_country: Option<Country> = Country::from_name(target_country_name.as_str());
    require!(target_country != None, ErrorCode::InvalidTargetCountryName);

    let opponent_country: Option<Country> = Country::from_name(opponent_country_name.as_str());
    require!(
        (opponent_country_name.as_str() == "")    // opponent_country_name can be empty 
            || (opponent_country != None),        // otherwise, it should be not None
        ErrorCode::InvalidOpponentCountryName
    );

    require!(
        (opponent_country_name.as_str() == "")    // opponent_country_name can be empty 
            || (target_country.unwrap() != opponent_country.unwrap()), // otherwise, it should be different from the target country
        ErrorCode::TargetAndOpponentCountriesAreSame
    );

    require!(
        description.len() >= DoveProject::MIN_DESCRIPTION,
        ErrorCode::TooShortDescription
    );
    require!(
        description.len() <= DoveProject::MAX_DESCRIPTION,
        ErrorCode::TooLongDescription
    );

    require!(
        video_link.len() <= DoveProject::MAX_HYPERLINK,
        ErrorCode::TooLongVideoLink
    );

    project.admin_pubkey = *ctx.accounts.admin.key;
    project.evidence_link = evidence_link;
    project.project_name = project_name;
    project.target_country_code = target_country.unwrap().to_string();
    if opponent_country_name.as_str() == "" {
        project.opponent_country_code = "".to_string();
    } else {
        project.opponent_country_code = opponent_country.unwrap().to_string();
    }
    project.description = description;

    project.created_date = DoveProject::get_now_as_unix_time();
    project.update_date = project.created_date;
    project.is_locked = false;
    project.is_deleted = false;
    project.video_link = video_link;
    project.amount_pooled = 0;
    project.amount_transferred = 0;
    project.last_date_transferred = project.created_date;
    project.bump = *ctx.bumps.get("dove_project").unwrap();
    Ok(())
}
