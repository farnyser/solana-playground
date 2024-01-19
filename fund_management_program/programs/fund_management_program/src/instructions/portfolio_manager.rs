use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::system_instruction;
use crate::errors::FundManagementError;
use crate::state::{Fund, FundState};

pub fn deposit(ctx: Context<crate::PortfolioManagerAccount>, amount: u64) -> Result<()> {
    require!(ctx.accounts.manager.is_signer, FundManagementError::InvalidPermission);

    let to_account =  &ctx.accounts.fund;
    let from_account = &ctx.accounts.manager;

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

    msg!("Deposited {} into fund: {}",
        amount,
        to_account.key());

    Ok(())
}

pub fn withdraw(ctx: Context<crate::PortfolioManagerAccount>, amount: u64) -> Result<()> {
    require!(ctx.accounts.manager.is_signer, FundManagementError::InvalidPermission);
    require!(ctx.accounts.fund.state == FundState::Open, FundManagementError::FundIsNotOpen);

    let from_account =  &ctx.accounts.fund;
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
        &[],
    )?;

    msg!("Withdrawn {} from fund: {}",
        amount,
        from_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct PortfolioManagerAccount<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
