use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{bs58, pubkey::Pubkey, system_instruction};
use std::str::FromStr;
use crate::types::*;

#[post("/send/sol")]
pub async fn send_sol(
    req: web::Json<SendSolRequest>,
) -> impl Responder {
    // Validate input
    if req.from.is_empty() || req.to.is_empty() || req.lamports == 0 {
        return HttpResponse::Ok().json(ApiResponse::<()> {
            success: false,
            data: (),
        });
    }
    let from_pubkey = match Pubkey::from_str(&req.from) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::<String> {
                success: false,
                data: "Invalid sender address".to_string(),
            });
        }
    };
    let to_pubkey = match Pubkey::from_str(&req.to) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(ApiResponse::<String> {
                success: false,
                data: "Invalid recipient address".to_string(),
            });
        }
    };
    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);
    let accounts = instruction.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();
    let instruction_data = bs58::encode(&instruction.data).into_string();
    let response_data = SendSolResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data,
    };
    println!("{}", instruction.program_id.to_string());
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
}