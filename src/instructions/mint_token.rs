use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{bs58, pubkey::Pubkey, signer::{keypair::Keypair, Signer}};
use spl_token::instruction::{initialize_mint, mint_to};
use std::str::FromStr;
use crate::types::*;


#[post("/token/mint")]
pub async fn mint_token(
    req: web::Json<MintToRequest>,
) -> impl Responder {
    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: "Invalid mint public key",
            });
        }
    };
    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: "Invalid destination public key",
            });
        }
    };
    let authority = match Pubkey::from_str(&req.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: "Invalid authority public key",
            });
        }
    };
    let token_program_id = spl_token::id();
    let instruction = match mint_to(
        &token_program_id,
        &mint,
        &destination,
        &authority,
        &[],
        req.amount,
    ) {
        Ok(instr) => instr,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: "Failed to create mint_to instruction",
            });
        }
    };
    let accounts = instruction
        .accounts
        .into_iter()
        .map(|account| AccountMetaData {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect::<Vec<AccountMetaData>>();
    let instruction_data = bs58::encode(&instruction.data).into_string();
    let response_data = MintToResponse {
        program_id: token_program_id.to_string(),
        accounts,
        instruction_data,
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
}