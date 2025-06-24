use actix_web::{HttpResponse, web};
use serde_json::json;
use std::sync::Mutex;

use crate::transaction::tx::Transaction;
use crate::Blockchain;

/// GET /chain
pub async fn get_chain(data: web::Data<Mutex<Blockchain>>) -> HttpResponse {
    let blockchain = data.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.chain)
}

/// POST /add_transaction
pub async fn add_transaction(
    tx: web::Json<Transaction>,
    data: web::Data<Mutex<Blockchain>>,
) -> HttpResponse {
    let mut blockchain = data.lock().unwrap();
    blockchain.add_transaction(tx.into_inner());
    HttpResponse::Created().json(json!({
        "status": "Transaction queued",
        "pending_pool_size": blockchain.pending_transactions.len()
    }))
}

/// POST /mine
pub async fn mine_block(data: web::Data<Mutex<Blockchain>>) -> HttpResponse {
    let mut blockchain = data.lock().unwrap();
    if blockchain.pending_transactions.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "No pending transactions to mine"
        }));
    }

    blockchain.mine_pending_transactions();
    let last_block = blockchain.chain.last().unwrap();

    HttpResponse::Ok().json(json!({
        "status": "Block successfully mined",
        "block_index": last_block.index,
        "transactions": last_block.transactions.len(),
        "hash": last_block.hash
    }))
}

/// GET /validate
pub async fn validate_chain(data: web::Data<Mutex<Blockchain>>) -> HttpResponse {
    let blockchain = data.lock().unwrap();
    let is_valid = blockchain.is_valid_chain();

    HttpResponse::Ok().json(json!({
        "valid": is_valid,
        "length": blockchain.chain.len()
    }))
}
