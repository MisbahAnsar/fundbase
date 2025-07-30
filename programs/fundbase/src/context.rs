// - `CreateCampaign` context:
//     - signer: user
//     - mutable `Campaign` PDA account
//     - system_program

// - `DonateToCampaign` context:
//     - signer: donor
//     - mutable `Campaign` account
//     - system_program

use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = user,
        space = Campaign::get_space(),
        seeds = [CAMPAIGN_PDA_SEED, user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>, // new acc will be generated here

    #[account(mut)]
    pub user: Signer<'info>, // user creating the campaign

    pub system_program: Program<'info, System>, // system program for account creation
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub donator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}