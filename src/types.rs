use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct AccountMetaData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaData>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct MintToRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MintToResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaData>,
    pub instruction_data: String,
}


#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageData {
    pub signature: String, // base64-encoded signature
    pub public_key: String, // base58-encoded public key
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenAccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<SendTokenAccountMeta>,
    pub instruction_data: String,
}