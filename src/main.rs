#![allow(deprecated, unused_imports)]
use std::{io::Result, str::FromStr};
use actix_web::{web, App, HttpServer};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, system_instruction, transaction::Transaction
};

// add the handlers module 
mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    // start the HttpServer 
    HttpServer::new(|| {
        App::new()
        .route("/balance/{pubkey}", web::get().to(handlers::get_balance))
        .route("/transaction", web::post().to(handlers::post))
        .route("/transaction/{pubkey}", web::get().to(handlers::get_transaction_history))
        .route("/transaction/full/{pubkey}", web::get().to(handlers::get_full_transaction_history))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
