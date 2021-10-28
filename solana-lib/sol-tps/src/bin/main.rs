use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use std::str::FromStr;
use sol_tps::Command;

fn main() -> anyhow::Result<()> {
    let matches = App::new("FxDex Command Lone Tools")
        .about("FunctionX")
        .version("v1.0")
        .arg(
            Arg::with_name("json_rpc_url")
                .short("u")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .help("URL for Solana's JSON RPC"),
        )
        .arg(
            Arg::with_name("program-id")
                .short("p")
                .long("program-id")
                .value_name("PROGRAM")
                .takes_value(true)
                .global(true)
                // .required(true)
                .help("DEX Program ID"),
        )
        .subcommand(market_sub_commands())
        .get_matches();
    let command;
    let rpc_client;
    let url;
    if let Some(rpc_url) = matches.value_of("json_rpc_url") {
        rpc_client =
            RpcClient::new_with_commitment(rpc_url.to_owned(), CommitmentConfig::confirmed());
        url = rpc_url.to_owned();
    } else {
        url = "http://127.0.0.1:8899".to_owned();
        rpc_client = RpcClient::new_with_commitment(
            "http://127.0.0.1:8899".to_owned(),
            CommitmentConfig::confirmed(),
        );
    }

    match matches.subcommand() {
        ("sol", Some(args)) => {
            command = parse_sol_sub_commands(args)?;
        }
        _ => command = None,
    };
    if let Some(cmd) = command {
        sol_tps::start(url, rpc_client, cmd)?;
    } else {
        println!("{}", matches.usage());
    }
    Ok(())
}


fn market_sub_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sol")
        .about("Solana tps test")
        .subcommand(
            SubCommand::with_name("create_keypair")
                .about("Create keypairs")
                .arg(
                    Arg::with_name("number")
                        .short("m")
                        .long("number")
                        .value_name("number")
                        .takes_value(true)
                        .help("Query DEX Market"),
                ),
        )
        .subcommand(
            SubCommand::with_name("mint_account")
                .about("mint_account")
                .arg(
                    Arg::with_name("number")
                        .short("m")
                        .long("number")
                        .value_name("number")
                        .takes_value(true)
                        .help("Query DEX Market"),
                ),
        )
        .subcommand(
            SubCommand::with_name("sol_transfer")
                .about("sol_transfer")
                .arg(
                    Arg::with_name("number")
                        .short("m")
                        .long("number")
                        .value_name("number")
                        .takes_value(true)
                        .help("Query DEX Market"),
                ),
        )
}

fn parse_sol_sub_commands(matches: &ArgMatches) -> anyhow::Result<Option<Command>> {
    Ok(match matches.subcommand() {
        ("create_keypair", Some(cmd)) => Some(Command::CreateKeypair {
            number: u64::from_str(cmd.value_of("number").unwrap()).unwrap(),
        }),

        ("mint_account", Some(cmd)) => Some(Command::MintAccount {
            number: u64::from_str(cmd.value_of("number").unwrap()).unwrap(),
        }),

        ("sol_transfer", Some(cmd)) => Some(Command::SolTransfer {
            number: u64::from_str(cmd.value_of("number").unwrap()).unwrap(),
        }),

        _ => None,
    })
}