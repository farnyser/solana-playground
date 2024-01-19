use anchor_lang::prelude::*;
use fixed::types::I80F48;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("EhHV9Fttbudzu9ARRMgWKJqRV4nFY2bTSb4pQYymE92M");

#[program]
pub mod fund_management_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, portfolio_manager: Pubkey) -> Result<()> {
        instructions::initialize(ctx, portfolio_manager)
    }

    // Investor deposit fund / will get a new minted token
    pub fn investor_deposit(ctx: Context<InvestorAccount>, amount_in_base_currency: u64) -> Result<()> {
        subscribe(ctx, amount_in_base_currency)
    }

    // Investor ask for withdrawal
    pub fn investor_withdraw_request(ctx: Context<InvestorAccount>, amount_in_token: u64) -> Result<()> {
        request_withdraw(ctx, amount_in_token)
    }

    // Trader deposit back profit / fund when needed
    pub fn portfolio_manager_deposit(ctx: Context<PortfolioManagerAccount>) -> Result<()> {
        deposit(ctx)
    }

    // Trader withdraw amount for trading
    pub fn portfolio_manager_withdraw(ctx: Context<PortfolioManagerAccount>) -> Result<()> {
        withdraw(ctx)
    }

    // Set fund value -> new token will be minted using this value
    pub fn fund_administrator_set_net_asset_value(ctx: Context<FundAdministratorAccount>, valuation: I80F48) -> Result<()> {
        set_nav(ctx, valuation)
    }

    // Send fund to investors who make a withdrawal request
    pub fn fund_administrator_handle_redemption(ctx: Context<FundAdministratorAccount>) -> Result<()> {
        handle_redemptions(ctx)
    }

    // Close fund (no more investment possible)
    pub fn fund_administrator_close_fund(ctx: Context<FundAdministratorAccount>) -> Result<()> {
        close(ctx)
    }
}