use anchor_lang::prelude::*;
use crate::state::Fund;

pub fn subscribe(ctx: Context<InvestorAccount>) -> anchor_lang::Result<()> {
    Ok(())
}

pub fn request_withdraw(ctx: Context<InvestorAccount>) -> anchor_lang::Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InvestorAccount<'info> {
    #[account(mut)] // TODO FAS should not be mut ?
    pub fund: Account<'info, Fund>,
    pub investor: Signer<'info>,
}
