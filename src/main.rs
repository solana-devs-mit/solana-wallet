#![allow(deprecated, unused_imports)]
use std::str::FromStr;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, system_instruction, transaction::Transaction
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Connect to Solana devnet
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(rpc_url);

    // Load sender keypair
    let keypair_path = "/home/yogesh/Downloads/wallet-keypair (4).json";
    let payer = read_keypair_file(keypair_path)
        .expect("Failed to read the keypair file!");

    println!("Using wallet: {}", payer.pubkey());

    // Receiver public key
    let yash_wallet_id = "9jEv35V6E8fYhyMzrK2bHHicsjTB9WzRYfFGrTYWQk1A";
    let yash_pubkey = Pubkey::from_str(yash_wallet_id)
        .expect("Invalid friend's public key!");

    // Amount to send (1 SOL)
    let amount_in_lamports = 1_000_000_000u64;

    // Check balance before sending
    let balance = client.get_balance(&payer.pubkey()).await?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    println!("Current balance: {:.2} SOL", balance_sol);
    if balance < amount_in_lamports {
        eprintln!("Not enough SOL to send.");
        return Ok(());
    }

    // Get recent blockhash
    let recent_blockhash = client
        .get_latest_blockhash()
        .await
        .expect("Failed to get the blockhash!");

    // Create transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &payer.pubkey(),
        &yash_pubkey,
        amount_in_lamports,
    );

    // Create transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Send and confirm
    match client.send_and_confirm_transaction(&transaction).await {
        Ok(sig) => println!("Transaction successful! Signature: {}", sig),
        Err(e) => eprintln!("Transaction failed: {:?}", e),
    }

   // Get sender's balance after transaction
match client.get_balance(&payer.pubkey()).await {
    Ok(balance) => println!("Sender's balance after tx: {:.2} SOL", balance as f64 / 1_000_000_000.0),
    Err(e) => eprintln!("Failed to get sender's balance: {:?}", e),
}

// Get receiver's balance after transaction
match client.get_balance(&yash_pubkey).await {
    Ok(balance) => println!("Receiver's balance after tx: {} SOL", balance as f64 / 1_000_000_000.0),
    Err(e) => eprintln!("Failed to get receiver's balance: {:?}", e),
}

return Ok(());
}
