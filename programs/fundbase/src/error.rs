// - Deadline must be in future
// - Target must be > 0
// - Index out of bounds
// - Overflow on donation

use anchor_lang::prelude::*;

#[error_code]
pub enum FundbaseError {
    #[msg("Campaign title exceeds maximum length")]
    TitleTooLong,
    
    #[msg("Campaign description exceeds maximum length")]
    DescriptionTooLong,
    
    #[msg("Campaign image URL exceeds maximum length")]
    ImageTooLong,
    
    #[msg("Campaign has too many donators")]
    TooManyDonators,
    
    #[msg("Donation amount exceeds maximum limit")]
    DonationAmountTooHigh,
    
    #[msg("Deadline must be in the future")]
    DeadlineInPast,
    
    #[msg("Target amount must be greater than zero")]
    TargetMustBePositive,
    
    #[msg("Index out of bounds for donators or donations")]
    IndexOutOfBounds,
    
    #[msg("Overflow occurred during donation processing")]
    OverflowError,

    #[msg("Insufficient funds in campaign account for donation")]
    InsufficientFunds,
}