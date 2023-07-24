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
    is_locked: bool,
)]
pub struct UpdateDoveProject<'info> {
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account()]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateDoveProject>,
    evidence_link: String,
    project_name: String,
    target_country_name: String,
    opponent_country_name: String,
    description: String,
    video_link: String,
    is_locked: bool,
) -> Result<()> {
    let project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        project.admin_pubkey == ctx.accounts.admin.key(),
        ErrorCode::InvalidUser
    );
    require!(!project.is_deleted, ErrorCode::DoveProjectIsAlreadyDeleted);
    require!(!project.is_locked, ErrorCode::DoveProjectIsLocked);
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
            || (opponent_country != None), // otherwise, it should be different from the target country
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

    require!(
        (project.evidence_link != evidence_link)
            || (project.project_name != project_name)
            || (project.target_country_code != target_country.unwrap().to_string())
            || (project.opponent_country_code != opponent_country.unwrap().to_string())
            || (project.description != description)
            || (project.video_link != video_link)
            || (project.is_locked != is_locked),
        ErrorCode::NoUpdateApplied,
    );

    project.evidence_link = evidence_link;
    project.project_name = project_name;
    project.target_country_code = target_country.unwrap().to_string();
    if opponent_country_name.as_str() == "" {
        project.opponent_country_code = "".to_string();
    } else {
        project.opponent_country_code = opponent_country.unwrap().to_string();
    }
    project.description = description;

    project.update_date = DoveProject::get_now_as_unix_time();
    project.is_locked = is_locked;
    project.video_link = video_link;
    Ok(())
}
