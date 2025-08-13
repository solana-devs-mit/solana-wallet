#![allow(deprecated, unused_imports)]
use std::{io::Result, str::FromStr};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, system_instruction, transaction::Transaction
};
use std::env;

// add the handlers module 
mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    // Get port from environment or default to 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = "0.0.0.0"; // <CHANGE> Changed from 127.0.0.1 to accept external connections

    // start the HttpServer 
    // cors configuration
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(3600);

        App::new()
        .wrap(cors)
        .route("/balance/{pubkey}", web::get().to(handlers::get_balance))
        .route("/transfer", web::post().to(handlers::transaction))
        .route("/transaction/{pubkey}", web::get().to(handlers::get_transaction_history))
        .route("/transaction/full/{pubkey}", web::get().to(handlers::get_full_transaction_history))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
