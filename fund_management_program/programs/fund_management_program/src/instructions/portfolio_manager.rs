use std::fmt::Debug;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::system_instruction;
use crate::errors::FundManagementError;
use crate::state::{Fund, FundState};

pub fn deposit(ctx: Context<crate::PortfolioManagerAccount>, amount: u64) -> Result<()> {
    require!(ctx.accounts.manager.is_signer, FundManagementError::InvalidPermission);

    let from_account = &ctx.accounts.manager;
    let to_account =  &ctx.accounts.vault;

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(&from_account.key,
                                                            &to_account.key(),
                                                            amount);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    msg!("Deposited {} into fund: {} (vault={})",
        amount,
        ctx.accounts.fund.key(),
        ctx.accounts.fund.fund_vault.key());

    Ok(())
}

pub fn withdraw(ctx: Context<crate::PortfolioManagerAccount>, amount: u64) -> Result<()> {
    require!(ctx.accounts.manager.is_signer, FundManagementError::InvalidPermission);
    require!(ctx.accounts.fund.state == FundState::Open, FundManagementError::FundIsNotOpen);

    let from_account =  &ctx.accounts.vault;
    let to_account = &ctx.accounts.manager;

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(&from_account.key(),
                                                            to_account.key,
                                                            amount);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&[
            ctx.accounts.fund.key().as_ref(),
            &[ctx.accounts.fund.fund_vault_bump]
        ]],
    )?;

    msg!("Withdrawn {} from fund: {} (vault={})",
        amount,
        ctx.accounts.fund.key(),
        ctx.accounts.fund.fund_vault.key());

    Ok(())
}

#[derive(Accounts)]
pub struct PortfolioManagerAccount<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    pub manager: Signer<'info>,
    /// CHECK: Only used to store SOL - TODO FAS is that safe ?
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
