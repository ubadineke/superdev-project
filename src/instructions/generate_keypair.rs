use std::str::FromStr;

use actix_web::{HttpResponse, Responder, dev::Response, post, web::Query};
use serde::{Deserialize, Serialize};
use solana_sdk::{bs58, signature::Keypair, signer::Signer};

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[post("/keypair")]
pub async fn generate_keypair() -> impl Responder {

    let keypair = Keypair::new();

    let pubkey = keypair.pubkey().to_string();

    let secret = bs58::encode(&keypair.to_bytes()).into_string();
    HttpResponse::Ok().json(
      ApiResponse {
        success: true,
        data: KeypairData { pubkey, secret },
    })
}
