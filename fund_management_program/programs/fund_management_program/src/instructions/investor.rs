use anchor_lang::prelude::*;
use crate::errors::FundManagementError;
use crate::state::{Fund, FundState};

pub fn subscribe(ctx: Context<InvestorAccount>, amount_in_base_currency: u64) -> anchor_lang::Result<()> {
    require!(ctx.accounts.fund.state == FundState::Open, FundManagementError::FundIsNotOpen);

    // TODO Transfer USDC/SOL from the investor into the fund management account
    // TODO Mint token to give to the investor (based on fund valuation)

    // TODO special case when fund valuation is 0 => use 1 token == 1 base ccy

    Ok(())
}

pub fn request_withdraw(ctx: Context<InvestorAccount>, amount_in_token: u64) -> anchor_lang::Result<()> {
    // TODO Transfer token from the investor into the fund management account
    // TODO Add amount+investor address to the pending redemptions list

    Ok(())
}

#[derive(Accounts)]
pub struct InvestorAccount<'info> {
    #[account(mut)] // TODO FAS should not be mut ?
    pub fund: Account<'info, Fund>,
    pub investor: Signer<'info>,
}
