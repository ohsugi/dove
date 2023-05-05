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
        evidence_link: String,
        project_name: String,
        target_country_name: String,
        opponent_country_name: String,
        description: String,
        video_link: String,
    ) -> Result<()> {
        create_dove_project::handler(
            ctx,
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
        evidence_link: String,
        project_name: String,
        target_country_name: String,
        opponent_country_name: String,
        description: String,
        video_link: String,
        is_locked: bool,
    ) -> Result<()> {
        update_dove_project::handler(
            ctx,
            evidence_link,
            project_name,
            target_country_name,
            opponent_country_name,
            description,
            video_link,
            is_locked,
        )
    }

    pub fn pull_dove_project(
        ctx: Context<PullDoveProject>,
        checked_amount_pooled: u64,
        checked_update_date: i64,
    ) -> Result<()> {
        pull_dove_project::handler(ctx, checked_amount_pooled, checked_update_date)
    }

    pub fn create_dove_user(
        ctx: Context<CreateDoveUser>,
        user_name: String,
        social_link: String,
        evidence_link: String,
        is_shown: bool,
    ) -> Result<()> {
        create_dove_user::handler(ctx, user_name, social_link, evidence_link, is_shown)
    }

    pub fn update_dove_user(
        ctx: Context<UpdateDoveUser>,
        user_name: String,
        social_link: String,
        evidence_link: String,
        is_shown: bool,
    ) -> Result<()> {
        update_dove_user::handler(ctx, user_name, social_link, evidence_link, is_shown)
    }

    pub fn delete_dove_user(ctx: Context<DeleteDoveUser>) -> Result<()> {
        delete_dove_user::handler(ctx)
    }

    pub fn create_dove_fund(
        ctx: Context<CreateDoveFund>,
        amount_pooled: u64,
        decision: f32,
        shows_user: bool,
        shows_pooled_amount: bool,
        shows_transferred_amount: bool,
    ) -> Result<()> {
        create_dove_fund::handler(
            ctx,
            amount_pooled,
            decision,
            shows_user,
            shows_pooled_amount,
            shows_transferred_amount,
        )
    }

    pub fn update_dove_fund(
        ctx: Context<UpdateDoveFund>,
        new_amount_pooled: u64,
        new_decision: f32,
        new_shows_user: bool,
        new_shows_pooled_amount: bool,
        new_shows_transferred_amount: bool,
    ) -> Result<()> {
        update_dove_fund::handler(
            ctx,
            new_amount_pooled,
            new_decision,
            new_shows_user,
            new_shows_pooled_amount,
            new_shows_transferred_amount,
        )
    }

    pub fn delete_dove_fund(ctx: Context<DeleteDoveFund>) -> Result<()> {
        delete_dove_fund::handler(ctx)
    }
}
