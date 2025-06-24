use actix_web::web::{self, ServiceConfig};

use super::handlers::{
    get_chain,
    add_transaction,
    mine_block,
    validate_chain,
    get_block_by_index,
    get_latest_block,
    get_transaction_pool,
    health_check,
};

/// Registers all HTTP routes for the OCOS Blockchain Node.
///
/// This includes blockchain inspection, transaction pool management,
/// mining, and basic node health endpoints.
///
/// # Arguments
/// * `cfg` - Mutable reference to the Actix-Web `ServiceConfig`.
pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg
        // --- Core Blockchain Routes ---
        .route("/chain", web::get().to(get_chain))
        .route("/chain/latest", web::get().to(get_latest_block))
        .route("/chain/block/{index}", web::get().to(get_block_by_index))
        .route("/validate", web::get().to(validate_chain))

        // --- Transaction Handling ---
        .route("/transactions", web::post().to(add_transaction))
        .route("/transactions/pool", web::get().to(get_transaction_pool))

        // --- Mining Operations ---
        .route("/mine", web::post().to(mine_block))

        // --- Node Utilities ---
        .route("/health", web::get().to(health_check));
}
