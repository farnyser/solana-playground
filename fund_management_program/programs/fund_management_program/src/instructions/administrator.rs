use anchor_lang::prelude::*;
use fixed::types::I80F48;
use crate::state::Fund;

pub fn set_nav(ctx: Context<FundAdministratorAccount>, valuation: I80F48) -> Result<()> {
    Ok(())
}

pub fn handle_redemptions(ctx: Context<FundAdministratorAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct FundAdministratorAccount<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    pub administrator: Signer<'info>,
}
