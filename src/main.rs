use std::env;

use actix_web::{App, HttpServer, web};

pub mod instructions;
pub use instructions::*;

pub mod types;
pub use types::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Solana Balance Server...");

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_address = format!("127.0.0.1:{}", port);
    println!("Listening on http://{}", bind_address);

    HttpServer::new(
        || App::new().service(generate_keypair)
        .service(create_token)
        .service(mint_token)
        .service(sign_message)
        .service(verify_message)
        .service(send_sol)
        .service(send_token) // .route("/get_balance", web::get().to(get_balance))
    )
    .bind(bind_address)?
    .run()
    .await
}
