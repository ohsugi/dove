use anchor_lang::prelude::*;

pub trait SizeDef {
    const HEADER_SIZE: usize = 8;
    const VEC_HEADER_SIZE: usize = 4;
    const PUBKEY_SIZE: usize = 32;
    const STRING_SIZE: usize = 4;
    const U64_SIZE: usize = 8;
    const I64_SZIE: usize = 8;
    const F32_SIZE: usize = 4;
    const BOOL_SIZE: usize = 1;
    const OPTION_MULTIPLIER: usize = 2;
    const BUMP_SIZE: usize = 1;
    const MAX_HYPERLINK: usize = 256;
    const MIN_AMOUNT_TO_POOLED: u64 = 0;
    const MAX_AMOUNT_TO_POOLED: u64 = 18400000000000000000;
    const MIN_AMOUNT_TO_TRANSFER: u64 = 0;
    const MAX_AMOUNT_TO_TRANSFER: u64 = 18400000000000000000;
    const MIN_PERCENTAGE: f32 = 0.0;
    const MAX_PERCENTAGE: f32 = 1.0;
    const SIZE: usize = 0;

    const ACCEPTABLE_AMOUNT_ERROR: u64 = 1000;
    const ACCEPTABLE_DATE_ERROR: u64 = 3000000;

    fn get_now_as_unix_time() -> i64 {
        return Clock::get().unwrap().unix_timestamp;
    }

    fn almost_equal_amount_pooled(amount1: u64, amount2: u64) -> bool {
        return amount1.abs_diff(amount2) < Self::ACCEPTABLE_AMOUNT_ERROR;
    }

    fn almost_equal_date(date1: i64, date2: i64) -> bool {
        return date1.abs_diff(date2) < Self::ACCEPTABLE_DATE_ERROR;
    }
}

#[account]
pub struct DoveCampaign {
    pub admin_pubkey: Pubkey,          // Admin's Wallet
    pub evidence_link: String,         // HTML link to prove Admin's identity
    pub campaign_name: String,         // Campaign Name
    pub target_country_code: String,   // Target country (ISO shortcode)
    pub opponent_country_code: String, // Opponent country (ISO shortcode), "" means no specific country
    pub description: String,           // Description and the usage of the transferred Solana
    pub created_date: i64,             // Campaign craetion date (as Unix Time)
    pub update_date: i64,              // Campaign update date (as Unix Time)
    pub is_locked: bool,               // If campaign is locked
    pub is_deleted: bool,              // If campaign is deleted
    pub video_link: String, // Video link to describe the campaign (intended youtube link)
    pub amount_pooled: u64, // The current pooled amount (as Lamports)
    pub amount_transferred: u64, // The amount transferred so far (as Lamports)
    pub decision: f32,      // The current decision for this campaign
    pub last_date_transferred: i64, // The last time pooled amount was transferred
    pub bump: u8,           // The bump number to avoid the duplicated PDA address
}

impl DoveCampaign {
    pub const MIN_ADMIN_NAME: usize = 3;
    pub const MAX_ADMIN_NAME: usize = 32;
    pub const MIN_CAMPAIGN_NAME: usize = 3;
    pub const MAX_CAMPAIGN_NAME: usize = 256;
    pub const MIN_COUNTRY_CODE: usize = 0;
    pub const MAX_COUNTRY_CODE: usize = 2;
    pub const MIN_DESCRIPTION: usize = 128;
    pub const MAX_DESCRIPTION: usize = 1024;
    pub const DECISION_THRESHOLD: f32 = 0.50;
}

impl SizeDef for DoveCampaign {
    const SIZE: usize = DoveCampaign::HEADER_SIZE // Header
        + DoveCampaign::PUBKEY_SIZE               // admin_pubkey
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_ADMIN_NAME * DoveCampaign::STRING_SIZE   // admin_name
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_HYPERLINK * DoveCampaign::STRING_SIZE    // evidence_link
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_CAMPAIGN_NAME * DoveCampaign::STRING_SIZE // campaign_name
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_COUNTRY_CODE * DoveCampaign::STRING_SIZE // target_country_code
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_COUNTRY_CODE * DoveCampaign::STRING_SIZE // opponent_country_code
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_DESCRIPTION * DoveCampaign::STRING_SIZE  // description
        + DoveCampaign::I64_SZIE       // created_date
        + DoveCampaign::I64_SZIE       // update_date
        + DoveCampaign::BOOL_SIZE      // is_locked
        + DoveCampaign::BOOL_SIZE      // is_deleted
        + DoveCampaign::VEC_HEADER_SIZE + DoveCampaign::MAX_HYPERLINK * DoveCampaign::STRING_SIZE    // video_link
        + DoveCampaign::U64_SIZE       // amount_pooled
        + DoveCampaign::U64_SIZE       // amount_transferred
        + DoveCampaign::F32_SIZE       // decision
        + DoveCampaign::I64_SZIE       // last_date_transferred
        + DoveCampaign::BUMP_SIZE      // bump
    ;
}

#[account]
pub struct DoveFund {
    pub campaign_pubkey: Pubkey,        // The target campaign pubkey
    pub user_pubkey: Pubkey,            // The founder's Wallet pubkey
    pub amount_pooled: u64,             // The current pooled amount
    pub amount_transferred: u64,        // The transferred amount so far
    pub decision: f32,                  // The decision percentage
    pub shows_user: bool,               // If the user will be shown on the campaign webpage
    pub shows_pooled_amount: bool, // If the user's pooled amount on the campaign webpage (as Lamports)
    pub shows_transferred_amount: bool, // If the user's transferred amount on the campaign webpage (as Lamports)
    pub created_date: i64,              // Fund craetion date (as Unix Time)
    pub update_date: i64,               // Fund update date (as Unix Time)
    pub bump: u8,                       // The bump number to avoid the duplicated PDA address
}

impl DoveFund {}

impl SizeDef for DoveFund {
    const SIZE: usize = DoveFund::HEADER_SIZE // Header
        + DoveFund::PUBKEY_SIZE    // campaign_pubkey
        + DoveFund::PUBKEY_SIZE    // user_pubkey
        + DoveFund::U64_SIZE       // amount_pooled
        + DoveFund::U64_SIZE       // amount_transferred
        + DoveFund::F32_SIZE       // decision
        + DoveFund::BOOL_SIZE      // shows_user
        + DoveFund::BOOL_SIZE      // shows_pooled_amount
        + DoveFund::BOOL_SIZE      // shows_transferred_amount
        + DoveFund::I64_SZIE       // created_date
        + DoveFund::I64_SZIE       // update_date
        + DoveFund::BUMP_SIZE      // bump
    ;
}

#[account]
pub struct DoveUser {
    pub user_pubkey: Pubkey,       // Wallet pubkey
    pub user_name: String,         // User name
    pub social_media_link: String, // Social media links of the user
    pub evidence_link: String,     // HTML link to prove own identity
    pub is_shown: bool,            // The profile will be shown on each campaign webpage
    pub created_date: i64,         // User craetion date (as Unix Time)
    pub update_date: i64,          // User update date (as Unix Time)
    pub bump: u8,                  // The bump number to avoid the duplicated PDA address
}

impl DoveUser {
    pub const MAX_USER_NAME: usize = 255;
    pub const MIN_USER_NAME: usize = 3;
    pub const MIN_HYPERLINK: usize = 4;
}

impl SizeDef for DoveUser {
    const SIZE: usize = DoveUser::HEADER_SIZE // Header
        + DoveUser::PUBKEY_SIZE    // user_pubkey
        + DoveUser::VEC_HEADER_SIZE + DoveUser::MAX_USER_NAME * DoveUser::STRING_SIZE // user_name
        + DoveUser::VEC_HEADER_SIZE + DoveUser::MAX_HYPERLINK * DoveUser::STRING_SIZE  // social_media_links
        + DoveUser::VEC_HEADER_SIZE + DoveUser::MAX_HYPERLINK * DoveUser::STRING_SIZE  // evidence_link
        + DoveUser::BOOL_SIZE      // is_shown
        + DoveUser::I64_SZIE       // created_date
        + DoveUser::I64_SZIE       // update_date
        + DoveUser::BUMP_SIZE;
}
