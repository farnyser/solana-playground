use anchor_lang::prelude::*;
use crate::state::Fund;

pub fn initialize(ctx: Context<Initialize>, portfolio_manager: Pubkey) -> Result<()> {
    ctx.accounts.fund.open(ctx.accounts.administrator.key(), portfolio_manager)?;

    msg!("Fund initialized with admin: {} and pm: {} (bump={})",
        ctx.accounts.administrator.key,
        portfolio_manager,
        ctx.bumps.fund);

    Ok(())
}

#[derive(Accounts)]
#[instruction(portfolio_manager: Pubkey)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = administrator,
        seeds = [administrator.key().as_ref(), portfolio_manager.as_ref()],
        bump,
        space = 8 + (32 + 32 + 1 + 8 + 16 + 8)
    )]
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub administrator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

