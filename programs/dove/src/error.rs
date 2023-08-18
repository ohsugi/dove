use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Too long Evidence Link.")]
    TooLongEvidenceLink,

    #[msg("Too short Campaign Name.")]
    TooShortCampaignName,
    #[msg("Too long Campaign Name.")]
    TooLongCampaignName,

    #[msg("Invalid Target Country name.")]
    InvalidTargetCountryName,

    #[msg("Invalid Opponent Country name.")]
    InvalidOpponentCountryName,

    #[msg("Target Country and Opponent Country are the same.")]
    TargetAndOpponentCountriesAreSame,

    #[msg("Too short campaign Description.")]
    TooShortDescription,
    #[msg("Too long campaign Description.")]
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

    #[msg("Dove Campaign is locked.")]
    DoveCampaignIsLocked,

    #[msg("Dove Campaign is not locked.")]
    DoveCampaignIsNotLocked,

    #[msg("Dove Campaign is already deleted.")]
    DoveCampaignIsAlreadyDeleted,

    #[msg("Pull Funds from the Target PDA is not allowed.")]
    PullFundsIsNotAllowed,

    #[msg("Inconsistent amount pooled between checked amount and Dove Campaign's pooled amount.")]
    InconsistentAmountPooled,

    #[msg("Invalid campaign for the Target PDA.")]
    InvalidCampaign,

    #[msg("Dove Fund was already transferred.")]
    DoveFundWasAlreadyTransferred,
}
