use anchor_lang::prelude::*;

pub fn deposit(ctx: Context<InvestorAccount>) -> anchor_lang::Result<()> { Ok(()) }

#[derive(Accounts)]
pub struct InvestorAccount {} // TODO move next to instruction ?
