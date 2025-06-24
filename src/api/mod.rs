//! # OCOS API Module
//!
//! This module exposes the external interface for interacting with the OCOS Blockchain Node,
//! including RESTful HTTP APIs and future WebSocket or JSON-RPC support.
//!
//! ## Submodules:
//! - `routes.rs`: Responsible for API endpoint registration
//! - `handlers.rs`: Logic implementations for each API route
//!
//! ## Roadmap:
//! - `ws.rs`: WebSocket-based subscription endpoints
//! - `rpc.rs`: JSON-RPC 2.0 standard interface
//! - `middleware.rs`: Authentication, logging, rate-limiting, etc.

pub mod routes;
pub mod handlers;

// Future expansion modules (uncomment when implemented)
// pub mod ws;
// pub mod rpc;
// pub mod middleware;

use actix_web::Scope;

/// Returns a scoped API router to be mounted on the main application.
///
/// This allows modular routing (e.g., under `/api/v1/` or similar).
pub fn scope() -> Scope {
    routes::register_routes()
}
