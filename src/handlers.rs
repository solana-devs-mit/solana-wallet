// here I'll write the handlers logic 
// GET POST handlers 
#![allow(unused_variables)]
use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{client, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::read_keypair_file, signer::Signer, system_instruction, transaction::Transaction};
use solana_transaction_status::UiTransactionEncoding;


fn get_rpc_url() -> String {
    std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string())
}

// GET req 
pub async fn get_balance(path: web::Path<String>) -> impl Responder {
    let pubkey_str = path.into_inner();
    let pubkey = match Pubkey::from_str(&pubkey_str) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().body("Invalid public key"),
    };

    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url.to_string());
    match client.get_balance(&pubkey).await {
        Ok(balance_lamports) => {
            let balance_sol = balance_lamports as f64 / LAMPORTS_PER_SOL as f64;
            HttpResponse::Ok().json(serde_json::json!({
                "pubkey": pubkey_str,
                "balance_sol": balance_sol
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

// now the POST logic where we send some test sol 
#[derive(Deserialize)]
pub struct TransactionParams {
    payer_id: String,
    reciever_id: String,
    amount_in_sol: f64
}

// POST req
pub async fn transaction(req: web::Json<TransactionParams>) -> impl Responder{
    // step 1: load the sender keypair (Private key)
    let payers_keypair = match read_keypair_file(&req.payer_id) {
        Ok(kp) => kp,
        Err(_) => return HttpResponse::BadRequest().body("Invalid keypair path"),
    };

    // step 2: parse the reciever's pubkey
    let reciever_pubkey = match Pubkey::from_str(&req.reciever_id) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().body("Invalid receiver public key")
    };

    // the amounts to be sent in SOL 
    let lamports = (req.amount_in_sol * LAMPORTS_PER_SOL as f64) as u64;
    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url.to_string());

    // get the recent blockhash 
    let recent_blockhash = match client.get_latest_blockhash().await {
        Ok(bh) => bh,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    // create the transfer instruction 
    let tx_instr = system_instruction::transfer(&payers_keypair.pubkey(), &reciever_pubkey, lamports);

    // create the transaction
    let tx = Transaction::new_signed_with_payer(&[tx_instr], Some(&payers_keypair.pubkey()), &[&payers_keypair], recent_blockhash) ;

    // send / make the transaction 
    match client.send_and_confirm_transaction(&tx).await {
        Ok(sig) => {
            // fetch balances after transactions 
            let sender_balance = 
            client.get_balance(&payers_keypair.pubkey()).await.unwrap_or_default() as f64 / LAMPORTS_PER_SOL as f64;

            let receiver_balance = client.get_balance(&reciever_pubkey).await.unwrap_or_default() as f64 / LAMPORTS_PER_SOL as f64;

            HttpResponse::Ok().json(serde_json::json!({
                "signature": sig.to_string(),
                "sender_balance_sol": sender_balance,
                "receiver_balance_sol": receiver_balance
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Transaction failed: {:?}", e)),
    }   
}  

// GET req 
pub async fn get_transaction_history(path: web::Path<String>) -> impl Responder {
    // now I'll have to get the wallet id / public key of the wallet and then pattern match the clint 
    let pubkey_str = path.into_inner();
    let pubkey = match Pubkey::from_str(&pubkey_str) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().body("Invalid public key"),
    };

    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url.to_string());

    // now call function call on the client,
    let transactions = match client.get_signatures_for_address(&pubkey).await {
        Ok(trans) => trans,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error {:?}", e)),
    };
    HttpResponse::Ok().json(transactions)
}   


// GET req
pub async fn get_full_transaction_history(path: web::Path<String>) -> impl Responder {

    // first check if url is correct 
    let pubkey_str = path.into_inner();
    let pubkey = match Pubkey::from_str(&pubkey_str) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().body("Invalid public key!")
    };

    // connect to solana devnet & client solana client
    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url.to_string());

    // get recent signature from public address via the .gettransactionviaaddress method 
    let signatures = match client.get_signatures_for_address(&pubkey).await {
        Ok(sigs) => sigs,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to fetch transactions: {:?}", e)),
    }; 

    // then have to loop through each signature and then fetch transactions 
    let mut transactions = Vec::new();
    for sig_info in signatures {
        let sig = match sig_info.signature.parse() {
            Ok(s) => s,
            Err(e) => {
                // skip invalid signature
                continue;
            }
        };

        match client.get_transaction(&sig, UiTransactionEncoding::JsonParsed).await {
            Ok(tx) => {
                transactions.push(tx);
            }
            Err(e) => {
                // Optionally, you can push error info or skip
                continue;
            }
        }
    }

    HttpResponse::Ok().json(transactions)
}
