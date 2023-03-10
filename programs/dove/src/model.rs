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

    fn get_now_as_unix_time() -> i64 {
        return Clock::get().unwrap().unix_timestamp;
    }
}

#[account]
pub struct DoveProject {
    pub admin_wallet: Pubkey,          // Admin's Wallet
    pub evidence_link: String,         // HTML link to prove Admin's identity
    pub project_name: String,          // Project Name
    pub target_country_code: String,   // Target country (ISO shortcode)
    pub opponent_country_code: String, // Opponent country (ISO shortcode), "" means no specific country
    pub description: String,           // Description and the usage of the transferred Solana
    pub created_date: i64,             // Project craetion date (as Unix Time)
    pub update_date: i64,              // Project update date (as Unix Time)
    pub is_effective: bool,            // If project is effective
    pub video_link: String,            // Video link to describe the project (intended youtube link)
    pub amount_pooled: u64,            // The current pooled amount (as Lamports)
    pub amount_transferred: u64,       // The amount transferred so far (as Lamports)
    pub decision: f32,                 // The current decision for this project
    pub bump: u8,
}

impl DoveProject {
    pub const MIN_ADMIN_NAME: usize = 3;
    pub const MAX_ADMIN_NAME: usize = 32;
    pub const MIN_PROJECT_NAME: usize = 3;
    pub const MAX_PROJECT_NAME: usize = 256;
    pub const MIN_COUNTRY_CODE: usize = 0;
    pub const MAX_COUNTRY_CODE: usize = 2;
    pub const MIN_DESCRIPTION: usize = 128;
    pub const MAX_DESCRIPTION: usize = 1024;
}

impl SizeDef for DoveProject {
    const SIZE: usize = DoveProject::HEADER_SIZE // Header
        + DoveProject::PUBKEY_SIZE               // admin_wallet
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_ADMIN_NAME * DoveProject::STRING_SIZE   // admin_name
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_HYPERLINK * DoveProject::STRING_SIZE    // evidence_link
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_PROJECT_NAME * DoveProject::STRING_SIZE // project_name
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_COUNTRY_CODE * DoveProject::STRING_SIZE // target_country_code
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_COUNTRY_CODE * DoveProject::STRING_SIZE // opponent_country_code
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_DESCRIPTION * DoveProject::STRING_SIZE  // description
        + DoveProject::I64_SZIE       // created_date
        + DoveProject::I64_SZIE       // update_date
        + DoveProject::BOOL_SIZE      // is_effective
        + DoveProject::VEC_HEADER_SIZE + DoveProject::MAX_HYPERLINK * DoveProject::STRING_SIZE    // video_link
        + DoveProject::U64_SIZE       // amount_pooled
        + DoveProject::U64_SIZE       // amount_transferred
        + DoveProject::F32_SIZE       // decision
        + DoveProject::BUMP_SIZE      // bump
    ;
}

#[account]
pub struct DoveFund {
    pub project_pubkey: Pubkey,         // The target project pubkey
    pub user_pubkey: Pubkey,            // The founder's Wallet pubkey
    pub amount_pooled: u64,             // The current pooled amount
    pub amount_transferred: u64,        // The transferred amount so far
    pub decision: f32,                  // The decision percentage
    pub shows_user: bool,               // If the user will be shown on the project webpage
    pub shows_pooled_amount: bool, // If the user's pooled amount on the project webpage (as Lamports)
    pub shows_transferred_amount: bool, // If the user's transferred amount on the project webpage (as Lamports)
    pub created_date: i64,              // Fund craetion date (as Unix Time)
    pub update_date: i64,               // Fund update date (as Unix Time)
    pub bump: u8,
}

impl DoveFund {}

impl SizeDef for DoveFund {
    const SIZE: usize = DoveFund::HEADER_SIZE // Header
        + DoveFund::PUBKEY_SIZE    // project_pubkey
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
    pub user_wallet: Pubkey,       // Wallet pubkey
    pub user_name: String,         // User name
    pub social_media_link: String, // Social media links of the user
    pub evidence_link: String,     // HTML link to prove own identity
    pub is_shown: bool,            // The profile will be shown on each project webpage
    pub amount_pooled: u64,        // The current pooled amount (as Lamports)
    pub amount_transferred: u64,   // The transferred amount so far (as Lamports)
    pub created_date: i64,         // User craetion date (as Unix Time)
    pub update_date: i64,          // User update date (as Unix Time)
    pub bump: u8,
}

impl DoveUser {
    pub const MAX_USER_NAME: usize = 255;
    pub const MIN_USER_NAME: usize = 3;
}

impl SizeDef for DoveUser {
    const SIZE: usize = DoveUser::HEADER_SIZE // Header
        + DoveUser::PUBKEY_SIZE    // user_wallet
        + DoveUser::VEC_HEADER_SIZE + DoveUser::MAX_USER_NAME * DoveUser::STRING_SIZE // user_name
        + DoveUser::VEC_HEADER_SIZE + DoveUser::MAX_HYPERLINK * DoveUser::STRING_SIZE  // social_media_links
        + DoveUser::VEC_HEADER_SIZE + DoveUser::MAX_HYPERLINK * DoveUser::STRING_SIZE  // evidence_link
        + DoveUser::BOOL_SIZE      // is_shown
        + DoveUser::U64_SIZE       // amount_pooled
        + DoveUser::U64_SIZE       // amount_transferred
        + DoveUser::I64_SZIE       // created_date
        + DoveUser::I64_SZIE       // update_date   
        + DoveUser::BUMP_SIZE;
}
