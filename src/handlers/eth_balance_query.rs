use actix_web::{get, http::StatusCode, web, Responder};
use ethers::prelude::*;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::utils::decimal_to_bigdecimal;
use crate::utils::u256_to_bigdecimal;

#[derive(Deserialize)]
struct Query {
    address: String,
    rpc: String,
    threshold: Decimal,
    comparison: Comparison,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Comparison {
    Gt,
    Ge,
    Eq,
    Le,
    Lt,
}

#[get("/query/ethBalance")]
async fn handle(query: web::Query<Query>) -> impl Responder {
    let parsed_address = query.address.parse::<Address>().unwrap();
    let rpc_provider = Provider::<Http>::try_from(query.rpc.clone()).unwrap();
    let threshold = decimal_to_bigdecimal(&query.threshold);

    let balance = rpc_provider
        .get_balance(parsed_address, None)
        .await
        .unwrap();
    let balance = u256_to_bigdecimal(&balance, 18);

    let condition_fulfilled = match query.comparison {
        Comparison::Gt => balance.gt(&threshold),
        Comparison::Ge => balance.ge(&threshold),
        Comparison::Eq => balance.eq(&threshold),
        Comparison::Le => balance.le(&threshold),
        Comparison::Lt => balance.lt(&threshold),
    };

    if condition_fulfilled {
        ("OK", StatusCode::OK)
    } else {
        ("ASSERTION_FAILED", StatusCode::BAD_REQUEST)
    }
}
