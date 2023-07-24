use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Too long Evidence Link.")]
    TooLongEvidenceLink,

    #[msg("Too short Project Name.")]
    TooShortProjectName,
    #[msg("Too long Project Name.")]
    TooLongProjectName,

    #[msg("Invalid Target Country name.")]
    InvalidTargetCountryName,

    #[msg("Invalid Opponent Country name.")]
    InvalidOpponentCountryName,

    #[msg("Target Country and Opponent Country are the same.")]
    TargetAndOpponentCountriesAreSame,

    #[msg("Too short project Description.")]
    TooShortDescription,
    #[msg("Too long project Description.")]
    TooLongDescription,

    #[msg("Too long Video link.")]
    TooLongVideoLink,

    #[msg("Url too short.")]
    TooShortUrl,
    #[msg("Url too long.")]
    TooLongUrl,

    #[msg("Too long URL in social Media links")]
    TooLongUrlInSocialMediaLinks,

    #[msg("Too small Amount Pooled")]
    TooSmallAmountPooled,
    #[msg("Too large Amount Pooled")]
    TooLargeAmountPooled,

    #[msg("Too small Decision value")]
    TooSmallDecision,
    #[msg("Too large Decision value")]
    TooLargeDecision,

    #[msg("Too short User Name.")]
    TooShortUserName,
    #[msg("Too long User Name.")]
    TooLongUserName,

    #[msg("Invalid User to Update the Target PDA.")]
    InvalidUser,

    #[msg("InsufficientFunds in the Target PDA.")]
    InsufficientFunds,

    #[msg("No change applied to the Target PDF.")]
    NoUpdateApplied,

    #[msg("Dove Project is locked.")]
    DoveProjectIsLocked,

    #[msg("Dove Project is not locked.")]
    DoveProjectIsNotLocked,

    #[msg("Dove Project is already deleted.")]
    DoveProjectIsAlreadyDeleted,

    #[msg("Pull Funds from the Target PDA is not allowed.")]
    PullFundsIsNotAllowed,

    #[msg("Inconsistent amount pooled between checked amount and Dove Project's pooled amount.")]
    InconsistentAmountPooled,

    #[msg("Invalid project for the Target PDA.")]
    InvalidProject,

    #[msg("Dove Fund was already transferred.")]
    DoveFundWasAlreadyTransferred,
}
