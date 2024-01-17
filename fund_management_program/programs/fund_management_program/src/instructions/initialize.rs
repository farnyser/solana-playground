use anchor_lang::prelude::*;
use crate::instructions::InvestorAccount;
use crate::state::Fund;

pub fn initialize(ctx: Context<Initialize>, portfolio_manager: Pubkey) -> anchor_lang::Result<()> {
    ctx.accounts.fund.open(ctx.accounts.administrator.key(), portfolio_manager);
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = administrator, space = 8 + 8)] // TODO FAS correct space
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub administrator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

