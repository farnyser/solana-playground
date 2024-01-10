use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    msg!("Hello world!");
    Ok(())
}

// Deployed at AbgXXgpcoDX8PUrNWUhR3xcUgQ1x9wdHBfKFMKZ67NrU