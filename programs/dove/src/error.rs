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

    #[msg("Invalid Is Effective flag.")]
    InvalidIsEffective,

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

    #[msg("Too small Amount Transferred")]
    TooSmallAmountTransferred,
    #[msg("Too large Amount Transferred")]
    TooLargeAmountTransferred,

    #[msg("Too small Decision value")]
    TooSmallDecision,
    #[msg("Invalid Decision value")]
    TooLargeDecision,

    #[msg("Invalid Shows User Flag.")]
    InvalidShowsUser,

    #[msg("Invalid Shows Pooled Amount flag.")]
    InvalidShowsPooledAmount,

    #[msg("Invalid Shows Transferred Amount flag.")]
    InvalidShowsTransferredAmount,

    #[msg("Too short User Name.")]
    TooShortUserName,
    #[msg("Too long User Name.")]
    TooLongUserName,

    #[msg("Invalid User to Update Dove Project.")]
    InvalidUserToUpdateDoveProject,
    #[msg("Debug Message")]
    DebugError,
}
