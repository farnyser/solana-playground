use std::fmt::{Debug, Formatter};
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use fund_management_client::{*};
use solana_sdk::signer::EncodableKey;
use solana_sdk::commitment_config::CommitmentConfig;
use clap::{arg, Parser, Command, Subcommand};
use solana_sdk::pubkey::Pubkey;
use anyhow::{Result, Error};
use solana_sdk::signature::Signature;

mod model;

#[derive(Debug, Parser)]
#[command(name = "fund_management_client")]
#[command(about = "Fund management CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "http://localhost:8899")]
    cluster_url: String,
}


#[derive(Debug, Subcommand)]
enum Commands {
    /// Get fund info
    #[command(arg_required_else_help = true)]
    FundInfo {
        /// The fund pubkey
        #[arg(value_parser = clap::value_parser!(model::pubkey_cli::PubkeyCli))]
        fund: model::pubkey_cli::PubkeyCli,
    },
    /// Create a new fund
    #[command(arg_required_else_help = true)]
    CreateFund {
        /// The portfolio manager pubkey
        #[arg(value_parser = clap::value_parser!(model::pubkey_cli::PubkeyCli))]
        portfolio_manager: model::pubkey_cli::PubkeyCli,
    },
    /// Set net asset value
    SetNav {
        /// The fund pubkey
        #[arg(value_parser = clap::value_parser!(model::pubkey_cli::PubkeyCli))]
        fund: model::pubkey_cli::PubkeyCli,
        /// The net asset valuation
        nav: f64,
    },
    /// Portfolio manager deposit asset into the fund vault
    PmDeposit {
        /// The fund pubkey
        #[arg(value_parser = clap::value_parser!(model::pubkey_cli::PubkeyCli))]
        fund: model::pubkey_cli::PubkeyCli,
        /// Amount to deposit (in fund base ccy)
        amount: u64,
    },
    /// Portfolio manager withdraw asset from the fund vault
    PmWithdraw {
        /// The fund pubkey
        #[arg(value_parser = clap::value_parser!(model::pubkey_cli::PubkeyCli))]
        fund: model::pubkey_cli::PubkeyCli,
        /// Amount to withdraw (in fund base ccy)
        amount: u64,
    },
}


fn main() {
    let args = Cli::parse();

    let url = args.cluster_url.parse::<Cluster>().unwrap();
    let wallet = Keypair::read_from_file(Path::new("/home/scawf/.config/solana/id.json")).unwrap();
    let payer = Rc::new(wallet);
    let client = Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::processed());

    let sig = match args.command {
        Commands::FundInfo { fund } => {
            println!(
                "Getting fund info for {}",
                fund.0
            );

            let fund = get_fund_info(&client, fund.0).unwrap();
            println!(" NAV: {}", fund.last_valuation);
            println!(" STATE: {:?}", fund.state);
            println!(" PM: {:?}", fund.portfolio_manager);
            println!(" ADMIN: {:?}", fund.fund_administrator);
            Result::<Signature>::Err(Error::msg("TODO"))
        },
        Commands::CreateFund { portfolio_manager } => {
            println!(
                "Creating fund with pm={}",
                portfolio_manager.0
            );

            create_new_fund(&client, &payer, &portfolio_manager.0)
        },
        Commands::SetNav { fund, nav } => {
            println!(
                "Setting nav for fund {} nav={}",
                fund.0,
                nav
            );
            set_fund_nav(&client, &payer, fund.0, nav)
        },
        Commands::PmDeposit { fund, amount } => {
            println!(
                "Deposit {} into fund {}",
                amount,
                fund.0,
            );
            deposit_into_fund(&client, &payer, fund.0, amount)
        },
        Commands::PmWithdraw { fund, amount } => {
            println!(
                "Withdraw {} from fund {}",
                amount,
                fund.0,
            );
            withdraw_from_fund(&client, &payer, fund.0, amount)
        },
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    };

    match sig {
        Ok(hash) => println!("-> {} !", hash),
        Err(err) => println!("ERROR {} !", err),
    }
}
