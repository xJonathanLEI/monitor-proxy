use actix_web::{App, HttpServer};
use std::{env, process};

use crate::handlers::*;

mod handlers;
mod utils;

const DEFAULT_PORT: u32 = 80;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Make listening port configurable through the PORT environment variable
    let port = match env::var("PORT") {
        Ok(env_port) => match env_port.parse() {
            Ok(env_port) => env_port,
            Err(err) => {
                eprintln!("Unable to parse port: {}", err);
                process::exit(1);
            }
        },
        Err(_) => DEFAULT_PORT,
    };

    HttpServer::new(|| App::new().service(eth_balance_query::handle))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
