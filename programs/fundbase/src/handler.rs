// - `pub fn create_campaign(...) -> Result<()>`:
//     - Initialize campaign fields
//     - Validate inputs (e.g. deadline in future)
// - `pub fn donate_to_campaign(...) -> Result<()>`:
//     - Append donor, amount
//     - Transfer lamports to campaign owner
//     - Update amount_collected

use crate::constants::*;
use crate::error::*;
use crate::context::*;
use anchor_lang::prelude::*;

pub fn create_campaign(
    ctx: Context<CreateCampaign>,
    title: String,
    description: String,
    target: u64,
    deadline: i64,
    image: String,
) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;

    if title.len() > MAX_TITLE_LENGTH {
        return Err(error!(FundbaseError::TitleTooLong));
    }
    if description.len() > MAX_DESCRIPTION_LENGTH {
        return Err(error!(FundbaseError::DescriptionTooLong));
    }
    if image.len() > MAX_IMAGE_LENGTH {
        return Err(error!(FundbaseError::ImageTooLong));
    }
    if deadline <= Clock::get()?.unix_timestamp {
        return Err(error!(FundbaseError::DeadlineInPast));
    }
    if target == 0 {
        return Err(error!(FundbaseError::TargetMustBePositive));
    }

    campaign.owner = ctx.accounts.user.key();
    campaign.title = title;
    campaign.description = description;
    campaign.target = target;
    campaign.deadline = deadline;
    campaign.amount_collected = 0;
    campaign.image = image;
    campaign.donators = Vec::new();
    campaign.donations = Vec::new();

    Ok(())
}

pub fn donate(
    ctx: Context<Donate>,
    amount: u64
) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;

    if campaign.donators.len() >= MAX_DONATORS {
        return Err(error!(FundbaseError::TooManyDonators));
    }
    if amount > MAX_DONATION_AMOUNT {
        return Err(error!(FundbaseError::DonationAmountTooHigh));
    }

    **ctx.accounts.donator.to_account_info().try_borrow_mut_lamports()? -= amount;
    **campaign.to_account_info().try_borrow_mut_lamports()? += amount;

    campaign.donators.push(ctx.accounts.donator.key());
    campaign.donations.push(amount);

    campaign.amount_collected = campaign
        .amount_collected
        .checked_add(amount)
        .ok_or(error!(FundbaseError::OverflowError))?;

    Ok(())
}

pub fn withdraw(
    ctx: Context<Withdraw>,
    amount: u64
) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;

    let campaign_balance = campaign.to_account_info().lamports();

    if amount > campaign_balance {
        return Err(error!(FundbaseError::InsufficientFunds));
    }

    **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.owner.try_borrow_mut_lamports()? += amount;

    Ok(())
}