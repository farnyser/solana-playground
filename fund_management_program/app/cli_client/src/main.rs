use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster};
use std::path::Path;
use std::rc::Rc;
use fund_management_cli_client::{create_new_fund};
use solana_sdk::signer::EncodableKey;
use solana_sdk::commitment_config::CommitmentConfig;

fn main() {
    println!("Starting client!");

    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );
    let wallet = Keypair::read_from_file(Path::new("/home/scawf/.config/solana/id.json")).unwrap();
    let payer = Rc::new(wallet);
    let client = Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::processed());

    println!("Creating fund...");
    let sig = create_new_fund(&client, &payer, &payer.pubkey());

    match sig {
        Ok(hash) => println!("-> {} !", hash),
        Err(err) => println!("ERROR {} !", err),
    }
}
