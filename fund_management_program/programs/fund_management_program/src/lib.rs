use anchor_lang::prelude::*;
use fixed::types::I80F48;
use instructions::*;

pub mod instructions;
mod state;

declare_id!("EhHV9Fttbudzu9ARRMgWKJqRV4nFY2bTSb4pQYymE92M");

#[program]
pub mod fund_management_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // Investor deposit fund / will get a new minted token
    pub fn investor_deposit(ctx: Context<InvestorAccount>) -> Result<()> { subscribe(ctx) }

    // Investor ask for withdrawal
    pub fn investor_withdraw_request(ctx: Context<InvestorAccount>) -> Result<()> { request_withdraw(ctx) }

    // Trader deposit back profit / fund when needed
    pub fn portfolio_manager_deposit(ctx: Context<PortfolioManagerAccount>) -> Result<()> { deposit(ctx) }

    // Trader withdraw amount for trading
    pub fn portfolio_manager_withdraw(ctx: Context<PortfolioManagerAccount>) -> Result<()> { withdraw(ctx) }

    // Set fund value -> new token will be minted using this value
    pub fn fund_administrator_set_net_asset_value(ctx: Context<FundAdministratorAccount>, valuation: I80F48) -> Result<()> {
        set_nav(ctx, valuation)
    }

    // Send fund to investors who make a withdrawal request
    pub fn fund_administrator_handle_redemption(ctx: Context<FundAdministratorAccount>) -> Result<()> { handle_redemptions(ctx) }
}

#[derive(Accounts)]
pub struct Initialize {}
