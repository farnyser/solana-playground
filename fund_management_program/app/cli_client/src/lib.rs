use std::error::Error;
use std::str::FromStr;
use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::{instruction::Instruction, program_pack::Pack, signer::{keypair::Keypair, Signer}, system_program, transaction::Transaction};
use solana_sdk::signature::Signature;
use anchor_client::{Client, Cluster};
use std::ops::Deref;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct HelloWorldInstruction {
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

    // let instruction_data = HelloWorldInstruction {};
    // let mut instruction_data_in_bytes: Vec<u8> = Vec::new();
    // instruction_data.serialize(&mut instruction_data_in_bytes)?;
    // let instruction = Instruction::new_with_bytes(fund_management_program::ID,
    //                                               &instruction_data_in_bytes,
    //                                               vec![]);
    //
    // let recent_blockhash = rpc_client.get_latest_blockhash()?;
    //
    // let  transaction =  Transaction::new_signed_with_payer(&[instruction],
    //                                                        Some(&signer_wallet.pubkey()),
    //                                                        &[signer_wallet, signer_wallet],
    //                                                        recent_blockhash);
    //
    // let sig = rpc_client.send_transaction(&transaction)?;
    // Ok(sig)
}