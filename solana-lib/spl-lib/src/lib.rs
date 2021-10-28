pub mod accounts;
pub mod errors;
pub mod state;
pub mod tokens;

use solana_sdk::pubkey::{ Pubkey, read_pubkey_file, write_pubkey_file };
use solana_sdk::instruction::{ Instruction };
use solana_sdk::transaction::{ Transaction };
use solana_sdk::signer::{ Signer };
use solana_sdk::signer::keypair::{ Keypair, read_keypair_file };
use solana_sdk::program_pack::{ Pack };

use solana_program::account_info::AccountInfo;
use spl_token::state::{ Mint, Account };

use solana_client::rpc_client::{ RpcClient };

pub fn create_mint(
    solana_client: &state::SolanaClient,
    filename: &str,
) -> Pubkey {
    // Create new Mint
    let mint_account: Keypair = Keypair::new();
    let mint_account_pubkey = mint_account.pubkey();
    println!("Special Token Mint: {}", mint_account_pubkey);

    let minimum_balance_for_rent_exemption = solana_client.client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    let create_account_instruction: Instruction =
        solana_sdk::system_instruction::create_account(
            &solana_client.pubkey,
            &mint_account_pubkey,
            minimum_balance_for_rent_exemption,
            Mint::LEN as u64,
            &spl_token::id(),
        );
    let initialize_mint_instruction: Instruction =
        spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint_account_pubkey,
            &solana_client.pubkey,
            None,
            9,
        ).unwrap();

    let (recent_blockhash, _fee_calculator) = solana_client.client.get_recent_blockhash().unwrap();

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &vec![
                create_account_instruction,
                initialize_mint_instruction,
            ],
            Some(&solana_client.pubkey),
            &[
                &mint_account,
                &solana_client.keypair,
            ],
            recent_blockhash,
        );

    let result = solana_client.client.send_transaction(&transaction);
    println!("'Create Account & Init Mint' Transaction Result: {:?}", result);

    if result.is_ok() {
        write_pubkey_file(filename, mint_account_pubkey).unwrap();
    }
    
    mint_account_pubkey
    // Special Token Mint: AuQ8pUSZu2gbwaHqrP5mrreGhQaHaqGZUBAtmq8hm3xf
    // 'Create Account & Init Mint' Transaction Result: Ok(3E8g6KuctAjbR3GwrxaHS6Ch26amZZnBWqbDwhLymCjx1e8UdrjnKFVfUCvUKCHNi3RpyJqie3hBrsvaKNHJDxmd)

}

pub fn create_mint2(
    solana_client: &state::SolanaClient,
    special_token_pubkey: &Pubkey,
    filename: &str,
) -> Pubkey {
    // If don't have Token Account mint to then create it
    let account_mint_to: Keypair = Keypair::new();
    let account_mint_to_pubkey: Pubkey = account_mint_to.pubkey();
    println!("New Account Mint To: {}", account_mint_to_pubkey);
    
    let create_account_instruction: Instruction =
        solana_sdk::system_instruction::create_account(
            &solana_client.pubkey,
            &account_mint_to_pubkey,
            solana_client.client.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap(),
            Account::LEN as u64,
            &spl_token::id(),
        );
    let initialize_account2_instruction: Instruction =
        spl_token::instruction::initialize_account2(
            &spl_token::id(),
            &account_mint_to_pubkey,
            &special_token_pubkey,
            &solana_client.pubkey,
        ).unwrap();

    let (recent_blockhash, _fee_calculator) = solana_client.client.get_recent_blockhash().unwrap();

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &vec![
                create_account_instruction,
                initialize_account2_instruction,
            ],
            Some(&solana_client.pubkey),
            &[
                &solana_client.keypair,
                &account_mint_to,
            ],
            recent_blockhash,
        );

    let result = solana_client.client.send_transaction(&transaction);
    println!("'Create Account' Transaction Result: {:?}", result);

    if result.is_ok() {
        write_pubkey_file(filename, account_mint_to_pubkey).unwrap();
    }

    account_mint_to_pubkey
    // New Account Mint To: AzY4jnfWXhrywfnSdDS9y8h3GJgGPBAhUpQcWkwxLC1A
    // 'Create Account' Transaction Result: Ok(36HqXAVrfjRWp8R8RwtQYfDWqCDZuCajT21MzAy5QvpH7RnWCqvYAx2SeBF8dyUuDLGkMvQguHFz34RVPAmgcEj)

}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    #[allow(dead_code)]
    use super::*;
    use spl_token::state::{ Mint, Account };

    use solana_client::nonce_utils;
    use solana_client::{
        rpc_client::{ RpcClient },
        nonce_utils::{ Error },
    };
    use solana_sdk::signer::keypair::{ Keypair, write_keypair_file, read_keypair_file };

    use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};
    use anyhow::{format_err, Result};
    use  solana_sdk::{
        signature::Signature,
        account_utils::StateMut,
        commitment_config::CommitmentConfig,
        program_pack::{ Pack },
        nonce::{
            state::{Data, Versions},
            State,
        },
    };


    const SOLANA_CLIENT_URL: &'static str = "http://127.0.0.1:8899";
    // const SOLANA_CLIENT_URL: &'static str = "http://localhost:8899";
    const WALLET_FILE_PATH: &'static str = "~/.config/solana/id.json";

    #[test]
    fn get_lamports() {
        //getMinimumBalanceForRentExemptio来查询，创建一个我们存的Message大小的 Account需要多少花费，然后通过系统的创建账号指令，创建一个Instruction
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());
        let minimum_balance_for_rent_exemption = client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();
        println!("{:?}", minimum_balance_for_rent_exemption); //1461600
        println!("{:?}",  Mint::LEN as u64); // 82

        let minimum_balance_for_rent_exemption = client.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap();
        println!("{:?}", minimum_balance_for_rent_exemption); //2039280
        println!("{:?}",  Account::LEN as u64); // 165
    }

    #[test]
    fn get_transaction() {
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());
        let signature = "zgAUhicUoLBi31mo9QvKQQgh6WQ55c2hMyAcb9BV8d7nhd7xH2DYHEzpb7yoiSTHNj8gf7qvinGqB6wNpN3zPRT".parse()
            .expect("Could not parse signature");
        let tx = client.get_transaction(&signature, UiTransactionEncoding::Base58).expect("Could not fetch transaction");
        println!("tx: {:?}", tx)
    }

    #[test]
    fn write_keypair() {
        for i in 1..100 {
            let s = format!("{}.keypair", i);
            let key: Keypair = Keypair::new();
            write_keypair_file(&key, s).expect("no error");
        }
    }

    #[test]
    fn create_account() {
        let solana_client = match state::SolanaClient::from_env() {
            Ok(client) => client,
            Err(e) => panic!("Error while initializing solana client: {:?}", e),
        };

        println!(
            "Running server using keypair with public key: {:?}",
            solana_client.pubkey
        );

        let filename: &str = "token_account2.pubkey";
        const SPECIAL_TOKEN_FILENAME: &'static str = "special_token.pubkey";
        
        let special_token_pubkey: Pubkey =
        if let Ok(pubkey) = read_pubkey_file(SPECIAL_TOKEN_FILENAME) {
            pubkey
        } else {
            create_mint(&solana_client, SPECIAL_TOKEN_FILENAME)
        };
        create_mint2(&solana_client, &special_token_pubkey, filename);
    }

    #[test]
    fn token_transfer() {
        let solana_client = match state::SolanaClient::from_env() {
            Ok(client) => client,
            Err(e) => panic!("Error while initializing solana client: {:?}", e),
        };

        println!("Wallet Balance: {}", solana_client.client.get_balance(&solana_client.pubkey).unwrap());

        let token_account_pubkey_s: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account.pubkey") {
            pubkey
        } else {
            return;
        };

        let token_account_pubkey_d: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account2.pubkey") {
            pubkey
        } else {
            return;
        };

        let mint_to_instruction: Instruction =
        spl_token::instruction::transfer(
            &spl_token::id(),
            &token_account_pubkey_s,
            &token_account_pubkey_d,
            &solana_client.pubkey,
            &[&solana_client.pubkey],
            200000000,
        ).unwrap(); //不需要输入合约地址

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
        println!("'Transfer' Transaction Result: {:?}", result);
        // 
    }

    //get_balance: query sol balance
    //get_token_account_balance: token account balance
    #[test]
    fn get_balance() { 
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());
        use std::str::FromStr;
        let mut token_account_pubkey_d: Pubkey = 
        if let Ok(pubkey) = Pubkey::from_str("B9MNRhmnj4XZuwvQE4Z4hfYgrWA2nHhwrRWEQiZin4Ya") {
            pubkey
        } else {
            println!("address parse error");
            return;
        };
        println!("{:?}", &token_account_pubkey_d);
        let result2 = client.get_balance(&token_account_pubkey_d);
        println!("balance2: {:?}", result2);


        let token_account_pubkey_s: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account.pubkey") {
            println!("{:?}", pubkey);
            pubkey
        } else {
            return;
        };

        let result = client.get_token_account_balance(&token_account_pubkey_s);
        println!("balance1: {:?}", result);


        let token_account_pubkey_s: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account2.pubkey") {
            println!("{:?}", pubkey);
            pubkey
        } else {
            return;
        };
        let result = client.get_token_account_balance(&token_account_pubkey_s);
        println!("balance2: {:?}", result);
    }

    #[test]
    fn sol_transfer() { 
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());

        let wallet_keypair = read_keypair_file(&*shellexpand::tilde(WALLET_FILE_PATH)).expect("Need a payer");
        let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
        println!("Wallet Pubkey: {}", wallet_pubkey);
        println!("Wallet Balance: {}", client.get_balance(&wallet_pubkey).unwrap());

        let token_account_pubkey_d: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account2.pubkey") {
            pubkey
        } else {
            return;
        };

        let transfer_instruction: Instruction =
        solana_sdk::system_instruction::transfer(
            &wallet_pubkey,
            &token_account_pubkey_d,
            200000000,
        );

        let (recent_blockhash, _fee_calculator) = client.get_recent_blockhash().unwrap();

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

        let result = client.send_transaction(&transaction);
        println!("'Transfer' Transaction Result: {:?}", result);
    }

    //TODO:
    #[test]
    fn trander_sol_from_token_account() { 
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());

        let wallet_keypair = read_keypair_file(&*shellexpand::tilde(WALLET_FILE_PATH)).expect("Need a payer");
        let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
        println!("Wallet Pubkey: {}", wallet_pubkey);
        println!("Wallet Balance: {}", client.get_balance(&wallet_pubkey).unwrap());

        let token_account_pubkey_d: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account2.pubkey") {
            pubkey
        } else {
            return;
        };

        let transfer_instruction: Instruction =
        solana_sdk::system_instruction::transfer(
            &token_account_pubkey_d,
            &wallet_pubkey,
            200000000,
        );

        let (recent_blockhash, _fee_calculator) = client.get_recent_blockhash().unwrap();

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

        let result = client.send_transaction(&transaction);
        println!("'Transfer' Transaction Result: {:?}", result);
    }

    #[test]
    fn test_get_account_data() {
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());

        let wallet_keypair = read_keypair_file(&*shellexpand::tilde(WALLET_FILE_PATH)).expect("Need a payer");
        let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
        println!("Wallet Pubkey: {}", wallet_pubkey);
        println!("Wallet Balance: {}", client.get_balance(&wallet_pubkey).unwrap());

        let token_account_pubkey_d: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account2.pubkey") {
            pubkey
        } else {
            return;
        };


        let data = client.get_account_data(&token_account_pubkey_d).expect("get account data");
        println!("event_q_data: {:?}", data);
    }

    #[test]
    fn test_get_account_for_token_account() {
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());

        let wallet_keypair = read_keypair_file(&*shellexpand::tilde(WALLET_FILE_PATH)).expect("Need a payer");
        let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
        println!("Wallet Pubkey: {}", wallet_pubkey);
        println!("Wallet Balance: {}", client.get_balance(&wallet_pubkey).unwrap());

        let token_account_pubkey_d: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("token_account2.pubkey") {
            pubkey
        } else {
            return;
        };

        let account =
        client.get_account(&token_account_pubkey_d).expect("get_account_with_commitment");
        println!("account: {:?}", account);
        //Account { lamports: 20002039280 data.len: 165 owner: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA executable: false rent_epoch: 193 data: bb585b13e03902b8661ceba509756b66566276da479cdaf62dcd9d81ed1f8c6496b88c293976d473d34c3cbf012271ebbc8c742eafea65fbbc7c9703eccfebeb }

        let data =
            client.get_account_with_commitment(&token_account_pubkey_d, CommitmentConfig::recent()).expect("get_account_with_commitment");
        println!("{:?}", data);
        //Response { context: RpcResponseContext { slot: 83723591 }, value: Some(Account { lamports: 20002039280 data.len: 165 owner: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA executable: false rent_epoch: 193 data: bb585b13e03902b8661ceba509756b66566276da479cdaf62dcd9d81ed1f8c6496b88c293976d473d34c3cbf012271ebbc8c742eafea65fbbc7c9703eccfebeb }) }
        
        let req_q_data = client
            .get_account_with_commitment(&token_account_pubkey_d, CommitmentConfig::recent()).unwrap()
            .value
            .ok_or(format_err!("Failed to retrieve account")).unwrap()
            .data;
        println!("{:?}", req_q_data);
    }

    #[test]
    fn test_get_account_for_mint_account() {
        let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());

        let wallet_keypair = read_keypair_file(&*shellexpand::tilde(WALLET_FILE_PATH)).expect("Need a payer");
        let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
        println!("Wallet Pubkey: {}", wallet_pubkey);
        println!("Wallet Balance: {}", client.get_balance(&wallet_pubkey).unwrap());

        let token_account_pubkey_d: Pubkey =
        if let Ok(pubkey) = read_pubkey_file("special_token.pubkey") {
            pubkey
        } else {
            return;
        };

        let account =
        client.get_account(&token_account_pubkey_d).expect("get_account_with_commitment");
        println!("account: {:?}", account);
        //account: Account { lamports: 1461600 data.len: 82 owner: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA executable: false rent_epoch: 193 data: 0100000096b88c293976d473d34c3cbf012271ebbc8c742eafea65fbbc7c9703eccfebebd02133040e0000000901000000000000000000000000000000000000 }
      //0100000096b88c293976d473d34c3cbf012271ebbc8c742eafea65fbbc7c9703eccfebebd02133040e0000000901000000000000000000000000000000000000

        let account2 = client
            .get_account_with_commitment(&token_account_pubkey_d, CommitmentConfig::recent());
        println!("{:?}", account2);
    }

    #[test]
    fn bincode_de() {
        let encoded = bincode::serialize(&(0u8, 10)).unwrap();
        println!("{:?}", encoded);

        let s = std::str::from_utf8(&encoded).unwrap();

        // let s = String::from_utf8(encoded).expect("Found invalid UTF-8");
        println!("result: {}", s);

        // let string = "00030000000a00";
        //
        // println!("{:?}", string.as_bytes());
        // let decoded: Option<String> = bincode::deserialize(string.as_bytes()).expect("data");
        // println!("{:?}", decoded);
    }

    #[test]
    fn array_test() {
        use arrayref::array_refs;
        let accounts:[u32; 5] = [1,2,3,4,5];

        let (
            &[],
            open_orders_accounts,
            &[
            ref market_acc,
            ref event_q_acc,
            ],
            _
        ) = array_refs![&accounts, 0; .. ; 2, 2];
        println!("array: {:?}", open_orders_accounts);
        println!("array: {:?}", market_acc);
        println!("array: {:?}", event_q_acc);
        println!("array: {:?}", accounts);
    }
}

//cargo test -- --nocapture