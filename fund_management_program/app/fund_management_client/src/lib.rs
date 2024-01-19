use solana_sdk::{signer::{keypair::Keypair, Signer}, system_program};
use solana_sdk::signature::Signature;
use anchor_client::{Client, Cluster};
use std::ops::Deref;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use fixed::types::I80F48;


pub fn get_fund_info<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    fund: Pubkey,
) -> Result<fund_management_program::state::Fund> {
    let program = client.program(fund_management_program::ID)?;
    let fund_account: fund_management_program::state::Fund = program.account(fund)?;
    Ok(fund_account)
}

pub fn create_new_fund<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: &Keypair,
    portfolio_manager: &Pubkey
) -> Result<Signature> {
    let program = client.program(fund_management_program::ID)?;

    let fund = Keypair::new();

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .signer(&fund)
        .accounts(fund_management_program::accounts::Initialize {
            fund: fund.pubkey(),
            administrator: signer_wallet.pubkey(),
            system_program: system_program::ID,
        })
        .args(fund_management_program::instruction::Initialize {
            portfolio_manager: *portfolio_manager
        })
        .send()?;

    Ok(sig)
}

pub fn set_fund_nav<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: &Keypair,
    fund: Pubkey,
    nav: f64
) -> Result<Signature> {
    let program = client.program(fund_management_program::ID)?;

    let nav = I80F48::from_num(nav);

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(fund_management_program::accounts::FundAdministratorAccount {
            fund: fund,
            administrator: signer_wallet.pubkey(),
        })
        .args(fund_management_program::instruction::FundAdministratorSetNetAssetValue {
            valuation: nav
        })
        .send()?;

    Ok(sig)
}