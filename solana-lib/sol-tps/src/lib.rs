#![deny(unaligned_references)]
#![allow(dead_code)]

use rand::rngs::OsRng;
use rand::{thread_rng, Rng};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::RpcRequest;
use solana_client::rpc_response::{RpcResult, RpcSimulateTransactionResult};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::Sysvar;
use solana_sdk::transaction::Transaction;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::process::exit;
use solana_gossip::gossip_service::{discover_cluster, get_multi_client, get_client};
use solana_sdk::client::AsyncClient;
use solana_streamer::socket::SocketAddrSpace;
use solana_sdk::signer::keypair::{ Keypair, write_keypair_file, read_keypair_file };
use solana_sdk::pubkey::{ Pubkey, read_pubkey_file, write_pubkey_file };
use std::thread;

const SOLANA_CLIENT_URL: &'static str = "127.0.0.1:1024";
const WALLET_FILE_PATH: &'static str = "~/.config/solana/id.json";

pub enum Command {
    CreateKeypair {
        number: u64,
    },

    MintAccount {
        number: u64,
    },


    SolTransfer {
        number: u64,
    },
}

pub fn start(rpc_url: String, rpc_client: RpcClient, cmd: Command) -> anyhow::Result<()> {

    let entry_point = solana_net_utils::parse_host_port(SOLANA_CLIENT_URL).unwrap();
    let nodes = discover_cluster(&entry_point, 1, SocketAddrSpace::Unspecified)
        .unwrap_or_else(|err| {
            eprintln!("Failed to discover {} nodes: {:?}", 1, err);
            exit(1);
        });

    let client = Arc::new(get_client(&nodes, &SocketAddrSpace::Unspecified));

    match cmd {
        Command::CreateKeypair {
            number,
        } => {
            for i in 0..number {
                let s = format!("{}.keypair", i);
                let key: Keypair = Keypair::new();
                write_keypair_file(&key, s).unwrap();

                let s_pub = format!("{}.pubkey", i);
                write_pubkey_file(s_pub.as_str(), key.pubkey()).unwrap();
            }
        }

        Command::MintAccount {
            number,
        } => {
            let wallet_keypair = read_keypair_file(&*shellexpand::tilde(WALLET_FILE_PATH)).expect("Need a payer");
            let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
            println!("Wallet Pubkey: {}", wallet_pubkey);
            println!("Wallet Balance: {}", rpc_client.get_balance(&wallet_pubkey).unwrap());

            for i in 0..number {
                let s = format!("{}.keypair", i);
                let keypair = read_keypair_file(&*shellexpand::tilde(&s)).unwrap();

                let transfer_instruction: Instruction =
                    solana_sdk::system_instruction::transfer(
                        &wallet_pubkey,
                        &keypair.pubkey(),
                        10000000u64,
                    );
                let (recent_blockhash, _fee_calculator) = rpc_client.get_recent_blockhash().unwrap();

                let transaction: Transaction =
                    Transaction::new_signed_with_payer(
                        &vec![
                            transfer_instruction,
                        ],
                        Some(&wallet_pubkey),
                        &[
                            &wallet_keypair,
                        ],
                        recent_blockhash,
                    );

                // let config = RpcSendTransactionConfig {
                //     skip_preflight: true,
                //     .. RpcSendTransactionConfig::default()
                // };

                let result = client.async_send_transaction(transaction);
                println!("'Transfer' Transaction Result:{:?}", result);
            }
        }

        Command::SolTransfer {
            number,
        } => {
            let mut handles = vec![];


            for i in 0..number {
                let s = format!("{}.keypair", i);
                let keypair = read_keypair_file(&*shellexpand::tilde(&s)).unwrap();


                let pubkey_path = format!("{}.pubkey", i + 1);
                let to: Pubkey =
                    if let Ok(pubkey) = read_pubkey_file(&pubkey_path) {
                        pubkey
                    } else {
                       "FzoWnVqUqpakUW7CbokFTWsnJr6uzphSWtei9JYDSF7e".parse()?
                    };


                let client2 = client.clone();
                let rpc_client = RpcClient::new(rpc_url.to_string());

                let handle = thread::spawn(move || {
                    let mut counter = 0;
                    let mut rng = rand::thread_rng();
                    for i in 0..100000 {
                        let y: u64 = rng.gen_range(1, 200);

                        let transfer_instruction: Instruction =
                            solana_sdk::system_instruction::transfer(
                                &keypair.pubkey(),
                                &to,
                                y,
                            );

                        let (recent_blockhash, _fee_calculator) = rpc_client.get_recent_blockhash().unwrap();

                        let transaction: Transaction =
                            Transaction::new_signed_with_payer(
                                &vec![
                                    transfer_instruction,
                                ],
                                Some(&keypair.pubkey()),
                                &[
                                    &keypair,
                                ],
                                recent_blockhash,
                            );

                        // let config = RpcSendTransactionConfig {
                        //     skip_preflight: true,
                        //     .. RpcSendTransactionConfig::default()
                        // };

                        let result = client2.async_send_transaction(transaction);
                        counter = counter + 1;
                        println!("'Transfer' Transaction Result:{}, {}, {:?}", keypair.pubkey(), counter, result);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        }
    };
    Ok(())
}

#[cfg(test)]
mod test {}
