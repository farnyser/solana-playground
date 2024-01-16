use std::error::Error;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;
use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::{
    instruction::Instruction,
    program_pack::Pack,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use solana_sdk::signature::Signature;

pub fn request_air_drop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_sol: u64) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL as f64)
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct HelloWorldInstruction {
}

pub fn call_hello_world(rpc_client: &RpcClient, signer_wallet: &Keypair) -> Result<Signature, Box<dyn Error>> {
    let PROGRAM_ID = Pubkey::from_str("AbgXXgpcoDX8PUrNWUhR3xcUgQ1x9wdHBfKFMKZ67NrU")?;
    let instruction_data = HelloWorldInstruction {};
    let mut instruction_data_in_bytes: Vec<u8> = Vec::new();
    instruction_data.serialize(&mut instruction_data_in_bytes)?;
    let instruction = Instruction::new_with_bytes(PROGRAM_ID,
                                                  &instruction_data_in_bytes,
                                                  vec![]);

    let recent_blockhash = rpc_client.get_latest_blockhash()?;

    let  transaction =  Transaction::new_signed_with_payer(&[instruction],
                                                           Some(&signer_wallet.pubkey()),
                                                           &[signer_wallet, signer_wallet],
                                                           recent_blockhash);

    let sig = rpc_client.send_transaction(&transaction)?;
    Ok(sig)
}