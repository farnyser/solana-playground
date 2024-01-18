use anchor_lang::prelude::*;
use fixed::types::I80F48;
use crate::errors::FundManagementError;
use crate::state::{Fund, FundState};

pub fn set_nav(ctx: Context<FundAdministratorAccount>, valuation: I80F48) -> Result<()> {
    require!(ctx.accounts.fund.state == FundState::Open, FundManagementError::FundIsNotOpen);
    require_keys_eq!(ctx.accounts.fund.fund_administrator, ctx.accounts.administrator.key(), FundManagementError::FundIsNotOpen);

    ctx.accounts.fund.last_valuation = valuation;
    ctx.accounts.fund.last_valuation_ts = Clock::get()?.unix_timestamp.try_into().unwrap();

    Ok(())
}

pub fn handle_redemptions(ctx: Context<FundAdministratorAccount>) -> Result<()> {
    Ok(())
}

pub fn close(ctx: Context<FundAdministratorAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct FundAdministratorAccount<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    pub administrator: Signer<'info>,
}
