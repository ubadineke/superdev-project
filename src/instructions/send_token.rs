use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{bs58, pubkey::Pubkey};
use spl_token::instruction::transfer;
use std::str::FromStr;
use crate::types::*;

#[post("/send/token")]
pub async fn send_token(
    req: web::Json<SendTokenRequest>,
) -> impl Responder {
    // Validate input
    if req.destination.is_empty() || req.mint.is_empty() || req.owner.is_empty() || req.amount == 0 {
        return HttpResponse::Ok().json(ApiResponse::<()> {
            success: false,
            data: (),
        });
    }
    let destination_pubkey = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::<String> {
                success: false,
                data: "Invalid destination address".to_string(),
            });
        }
    };
    let mint_pubkey = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::<String> {
                success: false,
                data: "Invalid mint address".to_string(),
            });
        }
    };
    let owner_pubkey = match Pubkey::from_str(&req.owner) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::<String> {
                success: false,
                data: "Invalid owner address".to_string(),
            });
        }
    };
    let token_program_id = spl_token::id();
    let instruction = match transfer(
        &token_program_id,
        &owner_pubkey,
        &destination_pubkey,
        &owner_pubkey,
        &[],
        req.amount,
    ) {
        Ok(instr) => instr,
        Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::<String> {
                success: false,
                data: "Failed to create transfer instruction".to_string(),
            });
        }
    };
    let accounts = instruction.accounts.iter().map(|meta| SendTokenAccountMeta {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
    }).collect();
    let instruction_data = bs58::encode(&instruction.data).into_string();
    let response_data = SendTokenResponse {
        program_id: token_program_id.to_string(),
        accounts,
        instruction_data,
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
}