mod error;
mod instructions;
mod model;

use anchor_lang::prelude::*;
pub use error::ErrorCode;
use instructions::*;

declare_id!("7ucMA2F1i5V1U2WDSEdwJhmdTc34aRZCxpbvkzF56XGs");

#[program]
pub mod dove {
    use super::*;

    pub fn create_dove_project(
        ctx: Context<CreateDoveProject>,
        admin_name: String,
        evidence_link: String,
        project_name: String,
        target_country_name: String,
        opponent_country_name: String,
        description: String,
        video_link: String,
    ) -> Result<()> {
        create_dove_project::handler(
            ctx,
            admin_name,
            evidence_link,
            project_name,
            target_country_name,
            opponent_country_name,
            description,
            video_link,
        )
    }

    pub fn update_dove_project(
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
        update_dove_project::handler(
            ctx,
            admin_name,
            evidence_link,
            project_name,
            target_country_name,
            opponent_country_name,
            description,
            video_link,
            is_effective,
            is_deleted,
        )
    }
}
