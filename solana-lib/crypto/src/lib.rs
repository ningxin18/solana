/*
Minted Ok(Keypair(Keypair { secret: SecretKey: [68, 211, 247, 214, 74, 106, 11, 49, 244, 238, 176, 5, 209, 240, 45, 160, 221, 131, 210, 195, 115, 102, 172, 220, 121, 225, 233, 185, 203, 162, 94, 162], public: PublicKey(CompressedEdwardsY: [65, 117, 173, 58, 32, 21, 149, 235, 242, 151, 142, 96, 60, 157, 83, 90, 1, 245, 239, 152, 165, 172, 69, 230, 99, 220, 249, 186, 132, 30, 158, 228]), EdwardsPoint{
        X: FieldElement51([270578207582189, 2131973742968173, 1906940198320206, 413430813510370, 2054329916012432]),
*/

#[cfg(test)]
mod tests {
    use ed25519_dalek::SecretKey;
    use solana_sdk::signature::{ Signer, Keypair };
    use solana_sdk::pubkey::{ Pubkey };

    #[test]
    fn secretkey() {
        let secret_key_bytes: [u8; 32] = [
            68, 211, 247, 214, 74, 106, 11, 49, 244, 238, 176, 5, 209, 240, 45, 160, 221, 131, 210, 195, 115, 102, 172, 220, 121, 225, 233, 185, 203, 162, 94, 162,
            ];
    
        let secret_key_pub_bytes: [u8; 64] = [
            234, 158, 122, 147, 63, 160, 164, 19, 228, 186, 223, 27, 146, 225, 158, 15, 247, 79, 194, 49, 153, 37, 81, 211, 171, 49, 37, 182, 1, 55, 5, 228,
            251, 218, 206, 134, 152, 246, 23, 179, 234, 101, 179, 50, 111, 241, 122, 240, 96, 208, 116, 91, 234, 106, 246, 88, 215, 120, 127, 133, 145, 165, 249, 197,
            ];
        let secret_key: SecretKey = SecretKey::from_bytes(&secret_key_bytes).expect("secret fail");
        println!("{:?}", secret_key);
    
        let secret_key2 = SecretKey::from_bytes(&secret_key_bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string())).expect("secret fail");
        println!("{:?}", secret_key2);
    
        let sol_keypair = Keypair::from_bytes(&secret_key_pub_bytes).unwrap();
        println!("{:?}", sol_keypair.to_base58_string());  //this is private key
        //2NpB6g4Je9MG8PRekh87yCs9KwFadQg7SQmaynR6ee9atfnhQAwCz6hrdtCfD7qaRomYqtnHpPhVqWtwpJ6RnV3R
    
        let keypair_from_base58 = Keypair::from_base58_string(&sol_keypair.to_base58_string());
        println!("{:?}", keypair_from_base58);  //same
    
        println!("-----------------------pubkey----------------------------");
        let pubkey: Pubkey = sol_keypair.pubkey();  //需要use trait Signer
        println!("{:?}", pubkey);


        println!("-----------------------pubkey----------------------------");
        let secret_key_pub_bytes2: [u8; 64] = [32,99,65,198,101,100,152,167,88,184,234,222,255,248,154,132,212,77,39,189,207,62,231,98,197,132,152,154,98,188,238,135,34,255,24,191,79,56,215,238,95,109,126,196,145,229,68,219,2,132,70,208,108,247,72,114,5,180,133,182,143,175,152,75];
        let sol_keypair = Keypair::from_bytes(&secret_key_pub_bytes2).unwrap();
        println!("{:?}", sol_keypair.to_base58_string());  //this is private key
        let pubkey: Pubkey = sol_keypair.pubkey();  //需要use trait Signer
        println!("{:?}", pubkey);


    }

    #[test]
    fn gen_order_id() {
        let seq_num: u64 = 10;
        let limit_price = 10_u64;
        let upper = (limit_price as u128) << 64;
        println!("左移: {:?}", upper); 

        println!("取反: {:?}", !seq_num); 
        let lower = seq_num;
        let order_id = upper | (lower as u128);
        println!("位或: {:?}", order_id);

        let order_id_bit =  (order_id >> 64) as u64;
        println!("原始值: {:?}", order_id_bit);
    }

    #[test]
    fn get_account_data() {
        let data: [u8; 68] = [187, 88, 91, 19, 224, 57, 2, 184, 102, 28, 235, 165, 9, 117, 107, 102, 86, 98, 118, 218, 71, 156, 218, 246, 45, 205, 157, 129, 237, 31, 140, 100, 150, 184, 140, 41, 57, 118, 212, 115, 211, 76, 60, 191, 1, 34, 113, 235, 188, 140, 116, 46, 175, 234, 101, 251, 188, 124, 151, 3, 236, 207, 235, 235, 0, 194, 235, 11];

    }
    
    #[test] 
    fn test_bincode() {
        let target: Option<String>  = Some("hello world".to_string());

        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        println!("{:?}", encoded);
        let s = String::from_utf8_lossy(&encoded[..]);
        println!("{:?}", s);

        let decoded: Option<String> = bincode::deserialize(&encoded[..]).unwrap(); 
        println!("{:?}", decoded);
    }
}

//cargo test -- --nocapture