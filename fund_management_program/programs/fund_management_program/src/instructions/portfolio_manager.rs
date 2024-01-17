use anchor_lang::prelude::*;
use crate::state::Fund;

pub fn deposit(ctx: Context<crate::PortfolioManagerAccount>) -> anchor_lang::Result<()> {
    Ok(())
}

pub fn withdraw(ctx: Context<crate::PortfolioManagerAccount>) -> anchor_lang::Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct PortfolioManagerAccount<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    pub manager: Signer<'info>,
}
