// - Declare the program module.
// - `#[program]` macro defining:
//     - `create_campaign` handler.
//     - `donate_to_campaign` handler.
//     - `get_campaigns` (optional view-like method).
//     - `get_donators` (optional view-like method).
// - Call respective handler functions (can be defined in `handlers.rs`).
// - Import context modules (e.g. `context::CreateCampaign`).

#![allow(deprecated)]

use anchor_lang::prelude::*;

declare_id!("Fundbase111111111111111111111111111111111111");

pub mod state;
pub mod context;
pub mod handler;
pub mod constants;
pub mod error;

use context::*;

#[program]
pub mod fundbase {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        title: String,
        description: String,
        target: u64,
        deadline: i64,
        image: String,
    ) -> Result<()> {
        handler::create_campaign(ctx, title, description, target, deadline, image)
    }

    pub fn donate(
        ctx: Context<Donate>,
        amount: u64
    ) -> Result<()> {
        handler::donate(ctx, amount)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64
    ) -> Result<()> {
        handler::withdraw(ctx, amount)
    }
}