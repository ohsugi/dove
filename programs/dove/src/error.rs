use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Too short Admin Name.")]
    TooShortAdminName,
    #[msg("Too long Admin Name.")]
    TooLongAdminName,

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
    #[msg("Invalid Is Deleted flag.")]
    InvalidDeleted,

    #[msg("Too long Video link.")]
    TooLongVideoLink,

    #[msg("Too long URL in social Media links")]
    TooLongUrlInSocialMediaLinks,

    #[msg("Too small Amount Transferred")]
    TooSmallAmountTransferred,
    #[msg("Too large Amount Transferred")]
    TooLargeAmountTransferred,

    #[msg("Invalid Decision value")]
    InvalidDecision,

    #[msg("Invalid Is Shown Flag.")]
    InvalidIsShown,

    #[msg("Invalid Shows Pooled Amount flag.")]
    InvalidShowsPooledAmount,

    #[msg("Invalid Shows Transferred Amount flag.")]
    InvalidShowsTransferredAmount,

    #[msg("Too short User Name.")]
    TooShortUserName,
    #[msg("Too long User Name.")]
    TooLongUserName,

    #[msg("Too small Amount Pooled")]
    TooSmallAmountPooled,
    #[msg("Too large Amount Pooled")]
    TooLargeAmountPooled,

    #[msg("Debug Message")]
    DebugError,
}
