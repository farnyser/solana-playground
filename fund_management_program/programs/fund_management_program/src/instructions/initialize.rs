use anchor_lang::prelude::*;
use crate::state::Fund;

pub fn initialize(ctx: Context<Initialize>, portfolio_manager: Pubkey) -> anchor_lang::Result<()> {
    ctx.accounts.fund.open(ctx.accounts.administrator.key(), portfolio_manager)?;

    msg!("Fund initialized with admin: {} and pm: {}",
        ctx.accounts.administrator.key,
        portfolio_manager);

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = administrator, space = 8 + 32 + 32 + 1 + 8 + 16 + 8)] // TODO FAS correct space (8 for hash of account + size of data struct)
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub administrator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

