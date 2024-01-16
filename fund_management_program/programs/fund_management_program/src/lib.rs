use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;

declare_id!("EhHV9Fttbudzu9ARRMgWKJqRV4nFY2bTSb4pQYymE92M");

#[program]
pub mod fund_management_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // Investor deposit fund / will get a new minted token
    pub fn investor_deposit(ctx: Context<InvestorAccount>) -> Result<()> { instructions::investor::deposit(ctx) }

    // Investor ask for withdrawal
    pub fn investor_withdraw_request(ctx: Context<InvestorAccount>) -> Result<()> { Ok(()) }

    // Trader deposit back profit / fund when needed
    pub fn portfolio_manager_deposit(ctx: Context<PortfolioManagerAccount>) -> Result<()> { Ok(()) }

    // Trader withdraw amount for trading
    pub fn portfolio_manager_withdraw(ctx: Context<PortfolioManagerAccount>) -> Result<()> { Ok(()) }

    // Set fund value -> new token will be minted using this value
    pub fn fund_administrator_set_net_asset_value(ctx: Context<FundAdministratorAccount>) -> Result<()> { Ok(()) }

    // Send fund to investors who make a withdrawal request
    pub fn fund_administrator_handle_redemption(ctx: Context<FundAdministratorAccount>) -> Result<()> { Ok(()) }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct PortfolioManagerAccount {}

#[derive(Accounts)]
pub struct FundAdministratorAccount {}
