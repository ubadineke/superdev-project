use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{bs58, pubkey::Pubkey, signer::{keypair::Keypair, Signer}};

use crate::types::*;
use solana_sdk::signature::Signature;

#[post("/message/sign")]
pub async fn sign_message(
    req: web::Json<SignMessageRequest>,
) -> impl Responder {

  
    // Validate fields
    if req.message.is_empty() || req.secret.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: (),
        });
    }

    // Decode secret key from base58
    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: (),
            });
        }
    };
    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: (),
            });
        }
    };

    // Sign the message
    let signature = keypair.sign_message(req.message.as_bytes());
    let signature_b64 = bs58::encode(signature.as_ref()).into_string();
    let public_key_b58 = keypair.pubkey().to_string();

    let data = SignMessageData {
        signature: signature_b64,
        public_key: public_key_b58,
        message: req.message.clone(),
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data,
    })
}