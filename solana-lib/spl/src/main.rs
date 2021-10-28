use solana_sdk::pubkey::{ Pubkey, read_pubkey_file };
use solana_sdk::instruction::{ Instruction };
use solana_sdk::transaction::{ Transaction };
use solana_sdk::signer::{ Signer };
use solana_sdk::signer::keypair::{ read_keypair_file };

use solana_client::rpc_client::{ RpcClient };

use spl_lib::{self, state::SolanaClient};


const SPECIAL_TOKEN_FILENAME: &'static str = "special_token.pubkey";
const TOKEN_ACCOUNT_FILENAME: &'static str = "token_account.pubkey";

fn main() {

    let solana_client = match SolanaClient::from_env() {
        Ok(client) => client,
        Err(e) => panic!("Error while initializing solana client: {:?}", e),
    };

    println!(
        "Running server using keypair with public key: {:?}",
        solana_client.pubkey
    );

    let special_token_pubkey: Pubkey =
        if let Ok(pubkey) = read_pubkey_file(SPECIAL_TOKEN_FILENAME) {
            pubkey
        } else {
            spl_lib::create_mint(&solana_client, SPECIAL_TOKEN_FILENAME)
        };
    let token_account_pubkey: Pubkey =
        if let Ok(pubkey) = read_pubkey_file(TOKEN_ACCOUNT_FILENAME) {
            pubkey
        } else {
            spl_lib::create_mint2(&solana_client, &special_token_pubkey, TOKEN_ACCOUNT_FILENAME)
        };
        
    // Mint some tokens
    let mint_to_instruction: Instruction =
        spl_token::instruction::mint_to(
            &spl_token::id(),
            &special_token_pubkey,
            &token_account_pubkey,
            &solana_client.pubkey,
            &[&solana_client.pubkey],
            20000000000,
        ).unwrap();

    let _mint_to_checked_instruction: Instruction =
        spl_token::instruction::mint_to_checked(
            &spl_token::id(),
            &special_token_pubkey,
            &token_account_pubkey,
            &solana_client.pubkey,
            &[&solana_client.pubkey],
            1000,
            9,
        ).unwrap();


    let (recent_blockhash, _fee_calculator) = solana_client.client.get_recent_blockhash().unwrap();

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &vec![
                mint_to_instruction,
            ],
            Some(&solana_client.pubkey),
            &[
                &solana_client.keypair,
            ],
            recent_blockhash,
        );

    let result = solana_client.client.send_transaction(&transaction);
    println!("'Mint To' Transaction Result: {:?}", result);
    //'Mint To' Transaction Result: Ok(3sHz8hfakdGkvX3JXw2f5tiF7a8oe6WYze6ZuWWeq7ChjjNNBvdLKxhWaavUuwmwKWdqEu6hpVRagaS4i152dUjF)
}
