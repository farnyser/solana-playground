use std::path::Path;
use solana_client::rpc_client::RpcClient;
use fund_management_cli_client::call_hello_world;
use solana_sdk::{
    signer::{keypair::Keypair, Signer},
};
use solana_sdk::signer::EncodableKey;

fn main() {
    println!("Starting client!");

    // let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let rpc_client = RpcClient::new("http://localhost:8899");
    let wallet = Keypair::read_from_file(Path::new("/home/scawf/.config/solana/id.json")).unwrap();

    // println!("SOL: {}", check_balance(&rpc_client, &pub_key).unwrap());
    //
    // request_air_drop(&rpc_client, &pub_key, 1).unwrap();
    //
    // println!("SOL: {}", check_balance(&rpc_client, &pub_key).unwrap());

    println!("Calling hello...");
    let sig = call_hello_world(&rpc_client, &wallet);
    match sig {
        Ok(hash) => println!("-> {} !", hash),
        Err(err) => println!("ERROR {} !", err),
    }
}
