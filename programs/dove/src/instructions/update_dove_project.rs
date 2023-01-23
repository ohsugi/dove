use crate::model::{DoveProject, SizeDef};
use anchor_lang::prelude::*;

use iso_country::Country;

#[derive(Accounts)]
#[instruction(
    admin_name: String,
    evidence_link: String,
    project_name: String,
    target_country_name: String,
    opponent_country_name: String,
    description: String,
    video_link: String,
    is_effective: bool,
    is_deleted: bool,
)]
pub struct UpdateDoveProject<'info> {
    #[account(mut)]
    pub dove_project: Account<'info, DoveProject>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateDoveProject>,
    admin_name: String,
    evidence_link: String,
    project_name: String,
    target_country_name: String,
    opponent_country_name: String,
    description: String,
    video_link: String,
    is_effective: bool,
    is_deleted: bool,
) -> Result<()> {
    let project: &mut Account<DoveProject> = &mut ctx.accounts.dove_project;

    require!(
        admin_name.len() >= DoveProject::MIN_ADMIN_NAME,
        TooShortAdminName
    );
    require!(
        admin_name.len() <= DoveProject::MAX_ADMIN_NAME,
        TooLongAdminName
    );

    require!(
        evidence_link.len() <= DoveProject::MAX_HYPERLINK,
        TooLongEvidenceLink
    );

    require!(
        project_name.len() >= DoveProject::MIN_PROJECT_NAME,
        TooShortProjectName
    );
    require!(
        project_name.len() <= DoveProject::MAX_PROJECT_NAME,
        TooLongProjectName
    );

    let target_country: Option<Country> = Country::from_name(target_country_name.as_str());
    require!(target_country != None, InvalidTargetCountryName);

    let opponent_country: Option<Country> = Country::from_name(opponent_country_name.as_str());
    require!(
        (opponent_country_name.as_str() == "")    // opponent_country_name can be empty 
            || (opponent_country != None), // otherwise, it should be different from the target country
        InvalidOpponentCountryName
    );

    require!(
        (opponent_country_name.as_str() == "")    // opponent_country_name can be empty 
            || (target_country.unwrap() != opponent_country.unwrap()), // otherwise, it should be different from the target country
        TargetAndOpponentCountriesAreSame
    );

    require!(
        description.len() >= DoveProject::MIN_DESCRIPTION,
        TooShortDescription
    );
    require!(
        description.len() <= DoveProject::MAX_DESCRIPTION,
        TooLongDescription
    );

    require!(
        video_link.len() <= DoveProject::MAX_HYPERLINK,
        TooLongVideoLink
    );

    project.admin_name = admin_name;
    project.evidence_link = evidence_link;
    project.project_name = project_name;
    project.target_country_code = target_country.unwrap().to_string();
    if opponent_country_name.as_str() == "" {
        project.opponent_country_code = "".to_string();
    } else {
        project.opponent_country_code = opponent_country.unwrap().to_string();
    }
    project.description = description;
    project.amount_transferred = 0.0;

    project.update_date = DoveProject::get_now_as_unix_time();
    project.is_effective = is_effective;
    project.is_deleted = is_deleted;
    project.video_link = video_link;
    Ok(())
}
