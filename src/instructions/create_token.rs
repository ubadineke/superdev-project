use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use solana_sdk::{bs58, pubkey::Pubkey, signer::{keypair::Keypair, Signer}};
use spl_token::instruction::initialize_mint;
use std::str::FromStr;
use crate::types::*;

#[derive(Deserialize)]
pub struct CreateTokenRequest{
  mintAuthority: String,
  mint: String,
  decimals: u8
}


// #[post("/token/create")]
// pub async fn create_token(
//     req: web::Json<CreateTokenRequest>,
// ) -> impl Responder {
//     let mint_authority = match Pubkey::from_str(&req.mint_authority) {
//         Ok(pubkey) => pubkey,
//         Err(_) => {
//             return HttpResponse::BadRequest().json(ApiResponse {
//                 success: false,
//                 data: "Invalid mint authority public key",
//             });
//         }
//     };
//     let mint = match Pubkey::from_str(&req.mint) {
//         Ok(pubkey) => pubkey,
//         Err(_) => {
//             return HttpResponse::BadRequest().json(ApiResponse {
//                 success: false,
//                 data: "Invalid mint public key",
//             });
//         }
//     };
//     let token_program_id = spl_token::id();
//     let instruction = match initialize_mint(
//         &token_program_id,
//         &mint,
//         &mint_authority,
//         None,
//         req.decimals,
//     ) {
//         Ok(instr) => instr,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ApiResponse {
//                 success: false,
//                 data: "Failed to create initialize mint instruction",
//             });
//         }
//     };
//     let accounts = instruction
//         .accounts
//         .into_iter()
//         .map(|account| AccountMetaData {
//             pubkey: account.pubkey.to_string(),
//             is_signer: account.is_signer,
//             is_writable: account.is_writable,
//         })
//         .collect::<Vec<AccountMetaData>>();
//     let instruction_data = bs58::encode(&instruction.data).into_string();
//     let response_data = CreateTokenResponse {
//         program_id: token_program_id.to_string(),
//         accounts,
//         instruction_data,
//     };

//     HttpResponse::Ok().json(ApiResponse {
//         success: true,
//         data: response_data,
//     })
// }



#[post("/token/create")]
pub async fn create_token(
    req: web::Json<CreateTokenRequest>,
) -> impl Responder {
    let mint_authority = match Pubkey::from_str(&req.mintAuthority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: "Invalid mint authority public key",
            });
        }
    };
    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: "Invalid mint public key",
            });
        }
    };
    let token_program_id = spl_token::id();
    let instruction = match initialize_mint(
        &token_program_id,
        &mint,
        &mint_authority,
        None,
        req.decimals,
    ) {
        Ok(instr) => instr,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: "Failed to create initialize mint instruction",
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
    let response_data = CreateTokenResponse {
        program_id: token_program_id.to_string(),
        accounts,
        instruction_data,
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
}