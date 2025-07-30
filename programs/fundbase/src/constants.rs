// - Max title length
// - Max description length
// - Max donators allowed
// - Campaign PDA seeds

pub const MAX_TITLE_LENGTH: usize = 100;
pub const MAX_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_IMAGE_LENGTH: usize = 200;
pub const MAX_DONATORS: usize = 100;
pub const MAX_DONATION_AMOUNT: u64 = 1_000_000_000; // 1 billion lamports
pub const CAMPAIGN_PDA_SEED: &[u8] = b"campaign";