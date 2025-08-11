// here I'll write the handlers logic 
// GET POST handlers 

use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

pub async fn get_balance(path: web::Path<String>) -> impl Responder {
    let pubkey_str = path.into_inner();
    let pubkey = match Pubkey::from_str(&pubkey_str) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().body("Invalid public key"),
    };

    let rpc_url = "https://api.devnet.solana.com".to_string();
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