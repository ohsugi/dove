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

    #[msg("Too small current Amount Pooled")]
    TooSmallCurrentAmountPooled,
    #[msg("Too large current Amount Pooled")]
    TooLargeCurrentAmountPooled,

    #[msg("Too small new Amount Pooled")]
    TooSmallNewAmountPooled,
    #[msg("Too large new Amount Pooled")]
    TooLargeNewAmountPooled,

    #[msg("Too small Decision value")]
    TooSmallDecision,
    #[msg("Too large Decision value")]
    TooLargeDecision,

    #[msg("Too small current Decision value")]
    TooSmallCurrentDecision,
    #[msg("Too large current Decision value")]
    TooLargeCurrentDecision,

    #[msg("Too small new Decision value")]
    TooSmallNewDecision,
    #[msg("Too large new Decision value")]
    TooLargeNewDecision,

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

    #[msg("Solana transaction failed.")]
    SolanaTransationFailed,

    #[msg("DoveProject has InsufficientFunds")]
    InsufficientFunds,

    #[msg("Invalid amount pooled in Project")]
    InvalidAmountPooledInProject,

    #[msg("Account conversion failed")]
    AccountConversionFailed,

    #[msg("No change applied to the Dove Fund")]
    NoChangeToDoveFund,

    #[msg("No change applied to the Dove Project")]
    NoChangeToDoveProject,

    #[msg("Invalid User to Update Dove Fund.")]
    InvalidUserToUpdateDoveFund,

    #[msg("Debug Message")]
    DebugError,
}
