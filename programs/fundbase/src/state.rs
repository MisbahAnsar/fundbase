// - `#[account]`-annotated `Campaign` struct:
//     - owner: Pubkey
//     - title: String
//     - description: String
//     - target: u64
//     - deadline: i64 (Unix timestamp)
//     - amount_collected: u64
//     - image: String
//     - donators: Vec<Pubkey>
//     - donations: Vec<u64>
// - Add `space` calculation method for `Campaign` account
// - Define maximum limits (e.g. string lengths, max donors).
use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Campaign {
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub target: u64,
    pub deadline: i64,
    pub amount_collected: u64,
    pub image: String,
    pub donators: Vec<Pubkey>,
    pub donations: Vec<u64>,
}

impl Campaign { 
    pub fn get_space() -> usize {
        8 + // discriminator
        32 + // owner
        4 + MAX_TITLE_LENGTH + // title
        4 + MAX_DESCRIPTION_LENGTH + // description
        8 + // target
        8 + // deadline
        8 + // amount_collected
        4 + MAX_IMAGE_LENGTH + // image
        4 + (MAX_DONATORS * 32) + // donators
        4 + (MAX_DONATORS * 8) // donations
    }
}